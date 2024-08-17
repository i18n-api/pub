use client::Uid;
use jarg::{jarg, json};
use re::ok;

use crate::{api, db};

pub async fn post(Uid(uid): Uid, jarg!(token_id): json!(u64)) -> re::msg!() {
  let ts = sts::sec();
  if m::tokenRefresh!(token_id, uid, ts) > 0 {
    db::sk::set(token_id, ts).await?;
    return ok!(api::Sk {
      v: db::sk::b64(uid, token_id, ts)
    });
  }

  ok!(())
}
