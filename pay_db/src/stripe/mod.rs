mod ing;
pub use ing::ing;
pub mod payment_intent;
pub mod setup_intent;
mod topup;
pub use topup::topup;
pub mod rm;
use std::{collections::HashSet, str::FromStr};

use aok::Result;
use dashmap::DashMap;
use mysql_async::prelude::FromRow;
pub use rm::rm;
use stripe::{Client, CreateCustomer, CreateSetupIntent, Customer, CustomerId, SetupIntent};

genv::def!(STRIPE_SK);

#[static_init::dynamic]
pub static STRIPE: Client = Client::new(STRIPE_SK::<String>());

#[static_init::dynamic]
pub static BRAND_NAME_ID: DashMap<Box<[u8]>, u16> = DashMap::new();

#[static_init::dynamic]
pub static BRAND_ID_NAME: DashMap<u16, String> = DashMap::new();

pub async fn brand_name_li(id_set: impl Iterator<Item = u16>) -> Result<Vec<(u16, String)>> {
  let mut not_exist = HashSet::new();
  let mut r = Vec::new();

  for id in id_set {
    if let Some(s) = BRAND_ID_NAME.get(&id) {
      r.push((id, s.clone()));
    } else {
      not_exist.insert(id);
    }
  }

  if !not_exist.is_empty() {
    for i in m::id_v_str("payBrand", not_exist).await? {
      BRAND_ID_NAME.insert(i.0, i.1.clone());
      BRAND_NAME_ID.insert(i.1.as_bytes().into(), i.0);
      r.push((i.0, i.1));
    }
  }

  Ok(r)
}

pub async fn brand_id(brand: &str) -> Result<u16> {
  let brand_bin = brand.as_bytes();
  if let Some(id) = BRAND_NAME_ID.get(brand_bin) {
    return Ok(*id);
  }

  let id = m::payBrandId(brand_bin).await?;
  BRAND_ID_NAME.insert(id, brand.to_owned());
  BRAND_NAME_ID.insert(brand_bin.into(), id);
  Ok(id)
}

pub fn payment_intent_id_bin(id: impl AsRef<str>) -> Result<Vec<u8>, b62::DecodeError> {
  let id = id.as_ref();
  b62::d(&id[3..])
}

pub fn customer_id_bin(id: impl AsRef<str>) -> Result<Vec<u8>, b62::DecodeError> {
  let id = id.as_ref();
  b62::d(&id[4..])
}

pub async fn customer(uid: u64) -> Result<Vec<u8>> {
  let uid_b64 = ub64::u64_b64(uid);
  Ok(
    if let Some::<Vec<u8>>(v) = m::q01!(format!("SELECT v FROM payStripeCustomer WHERE id={uid}")) {
      v
    } else {
      let customer = Customer::create(
        &STRIPE,
        CreateCustomer {
          name: Some(&uid_b64),
          // email: Some("test@async-stripe.com"),
          // description: Some(
          //   "A fake customer that is used to illustrate the examples in async-stripe.",
          // ),
          // metadata: Some(std::collections::HashMap::from([(
          //   String::from("id"),
          //   uid_b64,
          // )])),
          ..Default::default()
        },
      )
      .await?;
      let v = customer_id_bin(customer.id)?;
      m::e!(
        format!("INSERT INTO payStripeCustomer(id,v) VALUES ({uid},?)"),
        &v
      );
      v
    },
  )
}

pub async fn setup_intents(uid: u64) -> aok::Result<String> {
  let customer = format!("cus_{}", b62::e(customer(uid).await?));
  let setup_intents = SetupIntent::create(
    &STRIPE,
    CreateSetupIntent {
      customer: Some(CustomerId::from_str(&customer)?),
      ..Default::default()
    },
  )
  .await?;
  Ok(if let Some(client_secret) = setup_intents.client_secret {
    client_secret.as_str()[5..].into()
  } else {
    "".into()
  })
}

#[derive(Debug, FromRow)]
pub struct StripePayMethod {
  pub id: u64,
  pub brand_id: u16,
  pub exp: u16,
  pub status: u8,
  pub name: String,
}

pub async fn payment_method_li(uid: u64) -> aok::Result<Vec<StripePayMethod>> {
  Ok(m::q!(format!(
    "SELECT id,brand_id,exp,status,name FROM payStripe WHERE uid={uid} ORDER BY ts DESC"
  )))
}

pub mod bank {
  pub fn bancontact(t: &stripe::SetupAttemptPaymentMethodDetailsBancontact) -> Option<&str> {
    if let Some(r) = &t.bank_code {
      Some(r.as_str())
    } else {
      None
    }
  }

  pub fn ideal(t: &stripe::SetupAttemptPaymentMethodDetailsIdeal) -> Option<&str> {
    t.bank.map(|i| i.as_str())
  }
}
