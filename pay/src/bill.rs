use client::Uid;
use jarg::{jarg, json};

use crate::{api, db::month_bill};

pub async fn post(Uid(uid): Uid, jarg!(month): json!(i32)) -> re::msg!() {
  Ok(api::Bill {
    li: month_bill(uid, month)
      .await?
      .into_iter()
      .map(|(cid, kid, rid, day, n)| api::Item {
        cid: cid as _,
        kid,
        rid,
        day,
        n,
      })
      .collect(),
  })
}
