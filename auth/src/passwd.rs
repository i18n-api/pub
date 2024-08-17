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
    let p = r::R.pipeline();
    client
      .sign_in(&p, uid_bin, &header, &addr, fingerprint)
      .await?;
    user::pipeline(&p, uid_bin).await?;
    let (ver, lang, name) = user::ver_lang_name(p.last().await?);
    ing.await?;
    return Ok(api::User {
      id: uid,
      name,
      lang,
      ver,
    });
  }
  throw!(header, account, ACCOUNT_NOT_EXIST)
}
