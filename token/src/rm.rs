use client::Uid;
use intbin::u64_bin;
use jarg::{jarg, json};
use r::fred::interfaces::KeysInterface;
use xbin::concat;

use crate::K;

pub async fn post(Uid(uid): Uid, jarg!(token_id): json!(u64)) -> re::msg!() {
  if m::tokenRm!(uid, token_id) > 0 {
    () = r::R.del(concat!(K::TOKEN, u64_bin(token_id))).await?;
  }

  Ok(())
}
