use client::Uid;
use jarg::{jarg, json};
use r::fred::interfaces::HashesInterface;

use crate::K;

pub async fn post(Uid(uid): Uid, jarg!(token_id): json!(u64)) -> re::msg!() {
  if m::tokenRm!(uid, token_id) > 0 {
    () = r::R.hdel(K::TOKEN, intbin::u64_bin(token_id)).await?;
  }

  Ok(())
}
