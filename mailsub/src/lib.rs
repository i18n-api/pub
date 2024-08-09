// t3::api!();
mod _mod;
urlmod!();

#[allow(non_snake_case)]
pub mod K;
use t3::HeaderMap;
mod db;
use auth::db::host::id_by_header;
use bytes::Bytes;
use client::Client;
use lang::Lang;
use re::ok;

pub async fn post(header: HeaderMap, client: Client, body: Bytes) -> re::msg!() {
  let host_id = id_by_header(&header).await?;
  if let Some(uid) = client.uid().await? {
    m::e!(
      "INSERT INTO mailsubUser(host_id,uid)VALUES(?,?) ON DUPLICATE KEY UPDATE id=id",
      host_id,
      uid
    );
  } else {
    let (mail, lang): (String, String) = sonic_rs::from_slice(&body)?;
    let mail = mail.trim().to_lowercase();
    if !mail.is_empty() {
      let lang = if let Ok::<Lang, _>(lang) = lang.try_into() {
        lang as u16
      } else {
        0
      };
      m::e!(
        "INSERT INTO mailsubMail(host_id,mail,lang)VALUES(?,?,?) ON DUPLICATE KEY UPDATE lang=?",
        host_id,
        mail,
        lang,
        lang
      );
    }
  }

  ok!(())
}
