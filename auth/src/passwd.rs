use client::Client;
use intbin::u64_bin;
use jarg::{jarg, json};
use t3::{ConnectInfo, HeaderMap};

use crate::{
  api,
  db::{code, host, passwd},
  i18n, throw,
};

pub async fn post(
  ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
  header: HeaderMap,
  client: Client,
  jarg!(fingerprint, account, passwd, code): json!(String, String, String, String),
) -> re::msg!() {
  let account = xmail::norm(account);
  if !code::verify(i18n::RESET_PASSWORD, &account, &passwd, code) {
    throw!(header, code, CODE, INVALID)
  }
  let host_id = host::id_by_header(&header).await?;

  if let Some(uid) = m::authHostIdMailUid!(host_id, account) {
    let ing = trt::spawn(passwd::set(uid, passwd));
    let uid_bin = &u64_bin(uid)[..];
    let p = user::pipeline(uid_bin).await?;
    client
      .sign_in(&p, uid_bin, &header, &addr, fingerprint)
      .await?;
    let (ver, lang, name, ..): (Option<u64>, _, _, ()) = p.all().await?;
    let lang = user::lang::get(lang) as _;
    ing.await?;
    return Ok(api::User {
      id: uid,
      name,
      lang,
      ver: ver.unwrap_or(0),
    });
  }
  throw!(header, account, ACCOUNT_NOT_EXIST)
}
