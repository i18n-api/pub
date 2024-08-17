use client::Uid;
use jarg::{jarg, json};

use crate::db::stripe;

pub async fn post(Uid(uid): Uid, jarg!(id): json!(u64)) -> re::msg!() {
  stripe::rm(uid, id).await?;
  Ok(())
}
