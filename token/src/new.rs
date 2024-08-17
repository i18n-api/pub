use client::Uid;
use jarg::{jarg, json};

use crate::{api, db};

pub async fn post(Uid(uid): Uid, jarg!(name): json!(String)) -> re::msg!() {
  let s = name.trim();
  let name = &s[..s.len().min(32)];
  Ok::<api::Token, _>(db::new(uid, name).await?)
}
