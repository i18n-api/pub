use client::Client;
use intbin::u64_bin;
use r::R;
use t3::HeaderMap;

use crate::db;

pub async fn post(header: HeaderMap, client: Client, _json: String) -> re::msg!() {
  if let Some(uid) = client.uid().await? {
    let uid_bin = u64_bin(uid);
    let lang = i18n::header_bin(&header);
    db::lang::set(&*R, &uid_bin, lang).await?;
  }
  Ok(())
}
