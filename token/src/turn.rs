use client::Uid;
use jarg::{jarg, json};
use r::fred::interfaces::HashesInterface;

use crate::{db, K};

pub async fn post(Uid(uid): Uid, jarg!(id, enable): json!(u64, i8)) -> re::msg!() {
  let disable = enable == 0;
  let enable = if disable { 0 } else { 1 };
  let ts: Option<u64> = m::q01!(format!("CALL tokenTurn({uid},{id},{enable})"));
  if let Some(ts) = ts {
    let id_bin = &intbin::u64_bin(id)[..];
    () = if disable {
      r::R.hdel(K::TOKEN, id_bin).await?
    } else {
      db::sk::set_bin(id_bin, ts).await?;
    }
  }
  Ok(())
}
