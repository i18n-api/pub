use std::str::FromStr;

use aok::{Result, OK};
use stripe::{PaymentMethod, PaymentMethodId, StripeError};

use super::STRIPE;
use crate::Api;

/// TODO: 当用户支付所有欠款之后再调用该函数从 stripe 删除付款方式
pub async fn strip_rm(id: u64) -> Result<()> {
  if let Some::<Vec<u8>>(v) = m::q01!(format!("SELECT v FROM payStripeRmed WHERE id={id}")) {
    let pm_id = PaymentMethodId::from_str(&format!("pm_{}", b62::e(v)))?;
    loop {
      if let Err(err) = PaymentMethod::detach(&STRIPE, &pm_id).await {
        if let StripeError::Stripe(ref err) = err {
          if err.http_status == 400 {
            if let Some(msg) = &err.message {
              if msg.contains(" not attached ") {
                break;
              }
            }
          }
        }
        return Err(err.into());
      }
    }
  } else {
    return OK;
  }
  m::e!("DELETE FROM payStripeRmed WHERE id={id}");
  OK
}

pub async fn rm(uid: u64, id: u64) -> Result<()> {
  if m::payStripeRm(uid, id).await? == 0 {
    Api::pay_off(uid).await?;
  }

  Ok(())
}

pub async fn rm_by_id(payment_method_id: impl AsRef<str>) -> Result<()> {
  let v = b62::e(&payment_method_id.as_ref()[3..]);
  if !v.is_empty() {
    m::e!("DELETE FROM payStripe WHERE v=?", v);
  }
  OK
}
