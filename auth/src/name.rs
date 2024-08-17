use client::Client;
use intbin::u64_bin;
use jarg::{jarg, json};
use r::{
  fred::interfaces::KeysInterface,
  R,
};
use xbin::concat;

use crate::db::name;

pub async fn post(client: Client, jarg!(uid, name): json!(u64, String)) -> re::msg!() {
  let name = name::cut(&name).to_owned();
  client.uid_logined(uid).await?;

  let uid_bin = u64_bin(uid);
  let uid_bin_ = &uid_bin[..];
  let p = R.pipeline();
  () = p.incr(concat!(user::K::VER, uid_bin_)).await?;
  () = p
    .set(
      concat!(user::K::NAME, uid_bin_),
      name.as_bytes(),
      None,
      None,
      false,
    )
    .await?;
  () = p.last().await?;

  let client_id = client.id;
  tokio::spawn(async move {
    m::authNameLog(uid, name).await?;
    let user = ws::msg_user_by_uid_bin(uid_bin).await?;
    ws::to_other(client_id, uid, user).await;
    Ok::<_, aok::Error>(())
  });
  Ok(())
}
