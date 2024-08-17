use client::Uid;

use crate::db::stripe;

pub async fn post(Uid(uid): Uid) -> re::msg!() {
  Ok(stripe::setup_intents(uid).await?)
}
