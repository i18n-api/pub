use client::Uid;
use jarg::{jarg, json};

pub async fn post(Uid(uid): Uid, jarg!(id): json!(u64)) -> re::msg!() {
  let now = sts::sec();
  m::e!(format!(
    "UPDATE payStripe SET ts={now} WHERE uid={uid} AND id={id}"
  ));
  Ok(())
}
