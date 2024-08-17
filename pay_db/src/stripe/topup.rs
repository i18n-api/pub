use std::str::FromStr;

use aok::Result;
use stripe::{
  CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods,
  CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects, Currency, CustomerId, PaymentIntent,
  PaymentIntentStatus, PaymentMethodId,
};

use super::super::{
  // exchange::{self},
  stripe::STRIPE,
};
use crate::exchange;

// 欧元 貌似最多充值一万
pub const MAX: u64 = 1_000_000;
// 最少充值10
pub const MIN: u64 = 10_00;

// pub async fn payment_method_default(uid: u64) -> Result<Option<Vec<u8>>> {
//   Ok(m::q01!(format!(
//     "SELECT v FROM payStripe WHERE uid={uid} ORDER BY ts DESC LIMIT 1"
//   )))
// }

pub struct Topup {
  pub rate: u64,
  pub n: u64,
  pub status: PaymentIntentStatus,
}

pub async fn topup(uid: u64, n: u64, card: u64) -> Result<Option<Topup>> {
  let payment_mothod = m::q01!(format!(
    "SELECT v FROM payStripe WHERE uid={uid} AND id={card}"
  ));

  topup_by_payment_method(uid, n, payment_mothod).await
}

pub async fn topup_by_payment_method(
  uid: u64,
  n: u64,
  payment_method: Option<Vec<u8>>,
) -> Result<Option<Topup>> {
  let n = n.clamp(MIN, MAX);
  // let (customer, rate): (Option<Vec<u8>>, _) = trt::join!(
  //   m::q01(
  //     format!("SELECT v FROM payStripeCustomer WHERE id={uid}",),
  //     vec![],
  //   ),
  //   exchange::eur()
  // );
  let customer: Option<Vec<u8>> = m::q01(
    format!("SELECT v FROM payStripeCustomer WHERE id={uid}",),
    vec![],
  )
  .await?;
  let rate = exchange::BASE;

  if let Some(customer) = customer
    && let Some(payment_method) = payment_method
  {
    let mut opt = CreatePaymentIntent::new(n as _, Currency::USD);
    // let mut opt = CreatePaymentIntent::new(n as _, Currency::EUR);
    opt.payment_method = Some(PaymentMethodId::from_str(&format!(
      "pm_{}",
      b62::e(payment_method)
    ))?);

    opt.confirm = Some(true);
    opt.automatic_payment_methods = Some(CreatePaymentIntentAutomaticPaymentMethods {
      allow_redirects: Some(CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Never),
      enabled: true,
    });
    opt.customer = Some(CustomerId::from_str(&format!("cus_{}", b62::e(customer)))?);

    let pay = PaymentIntent::create(&STRIPE, opt).await?;

    let n = if pay.status == PaymentIntentStatus::Succeeded {
      pay.amount_received
    } else {
      pay.amount
    };

    let topup = Topup {
      n: n as _,
      rate,
      status: pay.status as _,
    };
    super::payment_intent::new(uid, &pay, rate).await?;
    return Ok(Some(topup));
  }

  Ok(None)
}
