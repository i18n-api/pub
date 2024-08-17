#![feature(let_chains)]
#![feature(macro_metavar_expr)]

t3::api!();
mod _mod;
urlmod!();

use client::Uid;
pub use db::{Api, K};
pub use pay_db as db;
use sonic_rs::from_slice;

pub type Bill = Vec<(u16, u64, u64, i32, i64)>;

pub async fn post(Uid(uid): Uid) -> re::msg!() {
  use crate::db::month_bill;

  let ing = trt::spawn(db::stripe::ing(uid));
  let mut month_li = db::bill_month(uid).await?;

  let month = sts::now_month();
  let day_range = sts::month_day(month);

  let json = m::payIndex(uid, day_range.start, day_range.end).await?;
  let (cash, li): (i64, Option<Bill>) = from_slice(&json)?;
  let li = if let Some(li) = li {
    month_li.push(month as _);
    li
  } else if !month_li.is_empty() {
    let m = month_li.last().unwrap();
    month_bill(uid, *m as _).await?
  } else {
    Vec::new()
  }
  .into_iter()
  .map(|(cid, kid, rid, day, n)| api::Item {
    cid: cid as _,
    kid,
    rid,
    day,
    n,
  })
  .collect::<Vec<_>>();

  month_li.reverse();
  Ok(api::BillIndex {
    month_li,
    cash,
    li,
    ing: ing
      .await?
      .into_iter()
      .map(|i| api::Ing { id: i.0, n: i.1 })
      .collect(),
  })
}
