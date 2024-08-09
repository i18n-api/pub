use client::Client;
use jarg::{jarg, json};

use crate::api;

// header: t3::HeaderMap,
pub async fn post(client: Client, jarg!(uid): json!(u64)) -> re::msg!() {
  client.uid_logined(uid).await?;

  // let host_id = host::id_by_header(&header)?;

  let mail = m::authUidMail!(uid);

  Ok(api::ConfMeta { mail })
}
