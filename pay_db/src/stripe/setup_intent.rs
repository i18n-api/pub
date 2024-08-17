use std::str::FromStr;

use stripe::{
  CustomerId,
  Expandable::Object,
  // Expandable::{Id, Object},
  ListSetupIntents,
  // PaymentMethod,
  PaymentMethodType::Card,
  // PaymentMethodType::{Bancontact, Card, Ideal, SepaDebit},
  SetupIntent,
  SetupIntentStatus::{Processing, RequiresAction, Succeeded},
};
use xstr::Join;

use super::{
  // bank,
  brand_id,
  customer,
  STRIPE,
};
use crate::Api;

pub struct NewArg {
  pub brand_id: u16,
  pub exp: u16,
  pub status: u8,
  pub ts: u64,
  pub kind: u16,
  pub name: String,
  pub v: Vec<u8>,
}

pub fn new_arg_format(arg: NewArg) -> String {
  let brand_id = arg.brand_id;
  let exp = arg.exp;
  let status = arg.status;
  let ts = arg.ts;
  let kind = arg.kind;
  let name = arg.name;
  let v = arg.v;

  format!(
    "{brand_id},{exp},{status},{ts},{},{},{}",
    m::b(&v),
    kind,
    m::s(name)
  )
}

pub enum SetupIntentState {
  Unexpected,
  Rmed,
  New(NewArg),
}

pub async fn new(setup_intent: SetupIntent) -> aok::Result<SetupIntentState> {
  if [Processing, Succeeded].contains(&setup_intent.status) {
    if let Some(Object(payment_method)) = setup_intent.payment_method {
      let status = setup_intent.status as u8;
      let ts = payment_method.created as u64;

      macro_rules! push {
            ($payment_method:expr,$brand:expr,$exp:expr,$name:expr) => {{
              let kind = payment_method.type_ as u16; // 必须取之前的 payment_method , 不然 ideal 会是 sepa

              if $payment_method.customer.is_none() {
                return Ok(SetupIntentState::Rmed);
              }

              let id = $payment_method.id.as_str();
              let v = b62::d(&id[3..])?;
              let brand_id = brand_id(&$brand).await?;
              return Ok(SetupIntentState::New(NewArg {
                brand_id,
                exp:$exp as u16,
                status,
                ts,
                kind,
                name:$name,
                v
              }));
              // arg_li.push(format!(
              //   "{uid},{},{},{status},{ts},{},{},{}",
              //   brand_id,
              //   $exp,
              //   m::b(&v),
              //   kind,
              //   m::s($name)
              // ))
            }};
          }

      // macro_rules! eur {
      //   ($method:ident) => {{
      //     if let Some(Object(setatt)) = setup_intent.latest_attempt {
      //       if let Some(m) = setatt.payment_method_details.$method {
      //         let bank = bank::$method(&m).unwrap_or(stringify!($method)).to_owned();
      //         let name = m.iban_last4.unwrap_or("".to_owned());
      //         if let Some(Id(ref id)) = m.generated_sepa_debit {
      //           let payment_method = PaymentMethod::retrieve(&STRIPE, &id, &[]).await?;
      //           push!(payment_method, bank, 0, name);
      //         }
      //       }
      //     }
      //   }};
      // }

      match payment_method.type_ {
        Card => {
          let card = payment_method.card.unwrap();
          let exp = sts::ym_n(card.exp_year as _, card.exp_month as _);
          push!(payment_method, card.brand, exp, card.last4);
        }
        // SepaDebit => {
        //   let sepa = payment_method.sepa_debit.unwrap();
        //   /*
        //   可以从这里获取银行代码的名字
        //   https://github.com/mdomke/schwifty/tree/main/schwifty
        //   */
        //   let mut li = Vec::with_capacity(2);
        //   for i in [
        //     sepa.country.unwrap_or("".to_owned()),
        //     sepa.last4.unwrap_or("".to_owned()),
        //   ] {
        //     if !i.is_empty() {
        //       li.push(i);
        //     }
        //   }
        //   push!(payment_method, "sepa", 0, li.join(" "));
        // }
        // Bancontact => {
        //   eur!(bancontact);
        // }
        // Ideal => {
        //   eur!(ideal);
        // }
        _ => {
          tracing::warn!(
            "未实现 payment_method 类型 : {}\n{:?}",
            payment_method.type_,
            payment_method
          );
        }
      }
    }
  } else if ![RequiresAction].contains(&setup_intent.status) {
    tracing::warn!(
      "未处理 setup_intent 状态 : {}\n{:?}",
      setup_intent.status,
      setup_intent
    );
  }
  Ok(SetupIntentState::Unexpected)
}

pub async fn pull(uid: u64) -> aok::Result<()> {
  let customer = format!("cus_{}", b62::e(customer(uid).await?));

  let mut starting_after = None;
  let mut limit = 2;

  let pre: Option<Vec<u8>> = m::q01!(format!(
    "SELECT v FROM (SELECT COALESCE((SELECT v FROM payStripe WHERE uid={uid} ORDER BY id DESC LIMIT 1),(SELECT v FROM payStripeRmed WHERE uid={uid} ORDER BY id DESC LIMIT 1)) AS v) AS t WHERE v IS NOT NULL"
  ));

  let mut arg_li = Vec::new();

  loop {
    let li = SetupIntent::list(
      &STRIPE,
      &ListSetupIntents {
        customer: Some(CustomerId::from_str(&customer)?),
        starting_after: starting_after.take(),
        limit: Some(limit),
        expand: &["data.latest_attempt", "data.payment_method"],
        ..Default::default()
      },
    )
    .await?;

    let data = li.data;

    if data.is_empty() {
      break;
    }

    let mut has_more = li.has_more;

    if has_more {
      starting_after = Some(data.last().unwrap().id.clone());
    }

    for setup_intent in data {
      use SetupIntentState::*;
      match new(setup_intent).await? {
        New(arg) => {
          if Some(&arg.v) == pre.as_ref() {
            has_more = false;
            break;
          }
          arg_li.push(arg);
        }
        Rmed => {
          has_more = false;
          break;
        }
        Unexpected => {}
      }
    }

    if !has_more {
      break;
    }

    limit = std::cmp::min(limit + 10, 100);
  }

  if !arg_li.is_empty() {
    let arg = arg_li
      .into_iter()
      .rev()
      .map(new_arg_format)
      .collect::<Vec<_>>()
      .join(format!("),({uid},"));

    let sql = format!(
      "INSERT INTO payStripe(uid,brand_id,exp,status,ts,v,kind,name)VALUES({uid},{arg})ON DUPLICATE KEY UPDATE status=VALUES(status),exp=VALUES(exp),name=VALUES(name)"
    );

    m::e!(sql);
    Api::pay_on(uid).await?;
  }
  Ok(())
}
