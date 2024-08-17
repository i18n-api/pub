use aok::{Result, OK};
use intbin::u64_bin;
use r::{fred::interfaces::HashesInterface, R};
use stripe::{Currency, PaymentIntent, PaymentIntentStatus};

use super::customer_id_bin;
use crate::{exchange, exchange::BASE, topup_cent, CID, K, KID};

pub const DUPLICATE_KEY: u16 = 1062;

pub const EUR_ID: u16 = 39;

pub async fn failed(uid: u64, indent: &PaymentIntent) -> Result<bool> {
  if [
    PaymentIntentStatus::Succeeded,
    PaymentIntentStatus::Processing,
  ]
  .contains(&indent.status)
  {
    return Ok(false);
  }
  let err = if let Some(err) = &indent.last_payment_error {
    if let Some(msg) = &err.message {
      msg.as_str()
    } else {
      ""
    }
  } else {
    ""
  };
  dbg!((&indent.id, &indent.status, err, uid, "todo 充值失败邮件"));
  Ok(true)
}

pub async fn topup(
  payment_indent_id: u64,
  uid: u64,
  amount_received: u64,
  rate: u64,
) -> Result<()> {
  let topup = amount_received * BASE / rate;
  // 手续费用触发器扣
  topup_cent(
    uid,
    CID::TOPUP as u16,
    KID::TOPUP::STRIPE as u64,
    payment_indent_id,
    topup,
  )
  .await?;
  dbg!((
    "todo 发送充值成功邮件",
    payment_indent_id,
    amount_received,
    uid,
    topup,
    rate
  ));
  OK
}

pub async fn new(uid: u64, indent: &PaymentIntent, rate: u64) -> Result<bool> {
  assert_eq!(Currency::USD, indent.currency);
  // assert_eq!(Currency::EUR, indent.currency);
  let amount = indent.amount;
  let amount_received = indent.amount_received as u64;
  let status = indent.status as u8;

  let v = super::payment_intent_id_bin(&indent.id)?;

  let sql = format!("INSERT INTO payStripeTopup(v,currency_id,n,recv,rate,status,uid)VALUES(?,{EUR_ID},{amount},{amount_received},{rate},{status},{uid})");

  match m::last_id(sql, m::arg!(v)).await {
    Ok(id) => {
      if amount_received > 0 {
        topup(id, uid, amount_received, rate).await?;
      } else if PaymentIntentStatus::Processing == indent.status {
        () = R
          .hset(
            &[K::PAY_ING, &u64_bin(uid)[..]].concat()[..],
            (u64_bin(id), u64_bin(amount as _)),
          )
          .await?;
      } else {
        failed(uid, indent).await?;
      }
    }
    Err(err) => {
      if let m::Error::Server(ref err) = err {
        /*
        插入重复数据, 就忽略, 表示已经存在
        https://en.oceanbase.com/docs/common-oceanbase-database-exchange::BASE0001031674
        */
        if err.code == DUPLICATE_KEY {
          return Ok(false);
        }
      }
      Err(err)?;
    }
  }

  Ok(true)
}

pub async fn update(indent: &PaymentIntent, v: &[u8]) -> Result<bool> {
  let amount_received = indent.amount_received as u64;
  let status = indent.status as u8;
  let (pre_id, updated, uid, rate): (u64, u8, u64, u64) = m::q1!(
    format!("CALL payStripeTopupUpdate(?,{amount_received},{status})"),
    v,
  );

  if pre_id == 0 {
    return Ok(false);
  }

  if updated > 0 && !failed(uid, indent).await? {
    if PaymentIntentStatus::Succeeded == indent.status {
      topup(pre_id, uid, amount_received, rate).await?;
      () = R
        .hdel(
          &[K::PAY_ING, &u64_bin(uid)[..]].concat()[..],
          u64_bin(pre_id),
        )
        .await?;
    } else {
      dbg!("todo ! redis zadd 在bill页面显示处理中");
    }
  }

  Ok(true)
}

pub async fn set(indent: PaymentIntent) -> Result<()> {
  if indent.status == PaymentIntentStatus::RequiresPaymentMethod
    && indent.last_payment_error.is_none()
  {
    return OK;
  }

  let v = &super::payment_intent_id_bin(&indent.id)?[..];

  if update(&indent, v).await? {
    return OK;
  }

  if let Some(customer) = &indent.customer {
    let uid: Option<u64> = m::q01(
      "SELECT id FROM payStripeCustomer WHERE v=?",
      m::arg!(customer_id_bin(customer.id())?),
    )
    .await?;
    let rate = exchange::BASE;
    // let (uid, rate): (Option<u64>, _) = trt::join!(
    //   m::q01(
    //     "SELECT id FROM payStripeCustomer WHERE v=?",
    //     m::arg!(customer_id_bin(customer.id())?)
    //   ),
    //   crate::exchange::eur()
    // );
    if let Some(uid) = uid {
      if new(uid, &indent, rate).await? {
        return OK;
      }
      update(&indent, v).await?;
    }
  }
  OK
}
