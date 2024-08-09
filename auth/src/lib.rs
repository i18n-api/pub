t3::api!();

urlmod!();
mod _mod;
pub mod db;
mod r#macro;
use aok::Result;

use crate::db::{bantld, SignIn};

#[allow(non_snake_case)]
pub mod K;
mod i18n;
use client::Client;
use intbin::u64_bin;
use jarg::{jarg, json};
use re::ok;
use t3::{ConnectInfo, HeaderMap};

pub const SIGN_UP: u8 = 0; // 注册
pub const SIGN_IN: u8 = 1; // 登录

pub async fn sign_in(
  client: &Client,
  id: u64,
  header: &HeaderMap,
  addr: &std::net::SocketAddr,
  fingerprint: &str,
) -> Result<api::User> {
  let client_uid = client.uid().await?;
  let id_bin = &u64_bin(id)[..];
  let p = user::pipeline(id_bin).await?;
  let (ver, lang, name) = if if let Some(uid) = client_uid {
    id != uid
  } else {
    true
  } {
    client
      .sign_in(&p, id_bin, header, addr, fingerprint)
      .await?;
    let li: (Option<_>, _, _, ()) = p.all().await?;
    (li.0, li.1, li.2)
  } else {
    p.all().await?
  };
  let lang = user::lang::get(lang) as _;
  Ok(api::User {
    id,
    ver: ver.unwrap_or(0),
    lang,
    name,
  })
}

#[arg::captcha]
pub async fn post(
  ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
  client: Client,
  header: HeaderMap,
  jarg!(fingerprint, action, account, password): json!(String, u8, String, String),
) -> re::msg!() {
  let account = xmail::norm(account);
  let host = &t3::origin_tld(&header)?;

  match db::sign_in(host, &account, &password).await? {
    SignIn::Ok(id) => {
      let user: api::User = sign_in(&client, id, &header, &addr, &fingerprint).await?;
      ok!(user)
    }
    SignIn::PasswdError => {
      if action == SIGN_UP {
        throw!(header, account, ACCOUNT_EXIST)
      }
      throw!(header, password, PASSWORD_ERROR)
    }
    SignIn::AccountNotExist => {
      if action == SIGN_UP {
        if let Some(p) = account.find('@') {
          if account.len() > p {
            let tld = xtld::host_tld(&account[p + 1..]);
            if bantld::is(tld).await? {
              throw!(header, account, BAN_MAIL)
            }
          }
        }
        return ok!(db::mail::host_send(i18n::SIGN_UP, &header, host, account, password).await?);
      }
      throw!(header, account, ACCOUNT_NOT_EXIST)
    }
  }
}
