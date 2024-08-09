#![feature(let_chains)]

// t3::api!();
mod _mod;
urlmod!();

#[allow(non_snake_case)]
pub mod K;
use jarg::{jarg, json};
use t3::HeaderMap;
mod db;
use auth::db::host::id_by_header;
use base64::prelude::{Engine, BASE64_URL_SAFE_NO_PAD as B64};
use client::Client;
use dashmap::DashMap;
use lang::Lang;
use re::ok;

#[static_init::dynamic]
pub static HOST_ID: DashMap<String, u32> = DashMap::new();

pub async fn post(
  header: HeaderMap,
  client: Client,
  jarg!(endpoint, auth, p256dh, lang): json!(String, String, String, String),
) -> re::msg!() {
  let host_id = id_by_header(&header).await?;
  let lang = if let Ok::<Lang, _>(lang) = lang.try_into() {
    lang as u16
  } else {
    0
  };

  if let Some(p) = endpoint.find("://")
    && let Some(auth) = auth.split("=").next()
    && let Some(p256dh) = p256dh.split("=").next()
  {
    let endpoint = &endpoint[p + 3..];
    if let Some(p) = endpoint.find("/")
      && (p + 1) < endpoint.len()
    {
      let host = &endpoint[..p];
      let endpoint = &endpoint[p + 1..];
      let auth = B64.decode(auth)?;
      let p256dh = B64.decode(p256dh)?;

      let endpoint_host_id = if let Some(id) = HOST_ID.get(host) {
        *id
      } else {
        let host_id;
        loop {
          let id: Option<u32> = m::q01!("SELECT id FROM webpushEndpointHost WHERE host=?", host);
          host_id = if let Some(id) = id {
            id
          } else if let Some(last_id) =
            m::last_id_or_none("INSERT INTO webpushEndpointHost(host)VALUES(?)", (host,)).await?
          {
            last_id as _
          } else {
            continue;
          };
          break;
        }
        HOST_ID.insert(host.into(), host_id);
        host_id
      };

      m::e!("INSERT INTO webpush(id,hostId,endpointHostId,endpoint,auth,p256dh,lang)VALUES(?,?,?,?,?,?,?) ON DUPLICATE KEY UPDATE endpointHostId=VALUES(endpointHostId),endpoint=VALUES(endpoint),auth=VALUES(auth),p256dh=VALUES(p256dh),lang=VALUES(lang)", client.id, host_id,endpoint_host_id, endpoint, auth, p256dh, lang);
    }
  }

  // if let Some(uid) = client.uid().await? {
  //   m::e!(
  //     "INSERT INTO mailsubUser(host_id,uid)VALUES(?,?) ON DUPLICATE KEY UPDATE id=id",
  //     host_id,
  //     uid
  //   );
  // } else {
  //   let (mail, lang): (String, String) = sonic_rs::from_slice(&body)?;
  //   let mail = mail.trim().to_lowercase();
  //   if !mail.is_empty() {
  //     m::e!(
  //       "INSERT INTO mailsubMail(host_id,mail,lang)VALUES(?,?,?) ON DUPLICATE KEY UPDATE lang=?",
  //       host_id,
  //       mail,
  //       lang,
  //       lang
  //     );
  //   }
  // }

  ok!(())
}
