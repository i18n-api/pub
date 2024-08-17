use client::Uid;
use jarg::{jarg, json};
use re::{form, ok};
use stripe::StripeError;

use crate::{api, db::stripe::topup};

pub async fn post(Uid(uid): Uid, jarg!(n, card): json!(u64, u64)) -> re::msg!() {
  match topup(uid, n, card).await {
    Ok(r) => {
      if let Some(r) = r {
        return ok!(api::Topuped {
          n: r.n,
          rate: r.rate,
          status: r.status as _,
        });
      }
    }
    Err(e) => {
      let e = e.downcast::<StripeError>()?;
      if let stripe::StripeError::Stripe(ref e) = e {
        if let Some(message) = &e.message {
          form::Error::throw("n", message)?;
        }
      }
      Err(e)?;
    }
  }
  ok!(())
}
