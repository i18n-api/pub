#![feature(let_chains)]
#![feature(macro_metavar_expr)]
use aok::Result;
use intbin::u64_bin;
use r::fred::interfaces::{KeysInterface, SetsInterface};
use xbin::concat;

#[allow(non_snake_case)]
pub mod K;
#[allow(non_snake_case)]
pub mod LUA;

mod cid;
pub use cid::CID;

#[allow(non_snake_case)]
pub mod KID;

pub mod exchange;
pub mod stripe;
pub use api::Api;
pub use bill_month::bill_month;
mod api;
mod bill_month;

pub const BASE: f64 = 100.0;
pub const MAX_DEBT: i64 = (-10.0 * BASE) as i64;

async fn _bill_new(uid: u64, cid: u16, kid: u64, rid: u64, amount: i64) -> Result<i64> {
  let ts = sts::sec();
  let reamin = m::payBillNew(uid, cid, kid, rid, amount, ts).await?;

  () = r::R
    .set(
      concat!(K::PAY_COST, u64_bin(uid)),
      &reamin.to_le_bytes()[..],
      None,
      None,
      false,
    )
    .await?;

  Ok(reamin)
}

pub async fn asset(uid: u64) -> Result<i64> {
  let r: Option<Vec<u8>> = r::R.get(concat!(K::PAY_COST, u64_bin(uid))).await?;
  if let Some(r) = r {
    if let Ok::<[u8; 8], _>(r) = r.try_into() {
      return Ok(i64::from_le_bytes(r));
    }
  }

  Ok(0)
}

pub async fn cost_cent(uid: u64, cid: u16, kid: u64, rid: u64, amount: u64) -> Result<i64> {
  let amount = amount as i64;
  let asset = _bill_new(uid, cid, kid, rid, -amount).await?;

  // 理论上, PAY_ON 被删除后不会再次有扣费, 每次都 srem 不影响性能
  if asset < MAX_DEBT {
    let uid_bin = intbin::u64_bin(uid);
    () = r::R.srem(K::PAY_ON, uid_bin).await?;
  }
  Ok(asset)
}

pub async fn topup_cent(uid: u64, cid: u16, kid: u64, rid: u64, amount: u64) -> Result<i64> {
  let amount = amount as i64;
  let asset = _bill_new(uid, cid, kid, rid, amount).await?;
  if asset >= MAX_DEBT {
    // 每次充值都pay_on一下, 充值不频繁, 不影响性能
    Api::pay_on(uid).await?;
  }
  Ok(asset)
}

pub async fn month_bill(uid: u64, month: i32) -> Result<Vec<(u16, u64, u64, i32, i64)>> {
  let mday = sts::month_day(month);
  Ok(m::q!(format!(
    "CALL payBill({uid},{},{})",
    mday.start, mday.end
  )))
}
