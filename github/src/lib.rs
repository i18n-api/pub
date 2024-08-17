#![feature(let_chains)]
use axum::{
  extract::{Query, Request},
  response::Redirect,
};
use client::Client;
use jarg::{jarg, json};
use pay_db::{BASE, CID, KID};
use serde::Deserialize;
use t3::IntoResponse;
use tokio::try_join;

t3::api!();

mod _mod;
urlmod!();

#[allow(non_snake_case)]
pub mod K;
mod db;

use crate::db::{
  github_url, State, FOLLOW_REPO, FOLLOW_USER, GITHUB_STAR_ID, GITHUB_STAR_SK, SCOPE_LI,
};

pub const BONUS: u64 = (50.0 * BASE) as _;

// POST

pub async fn post(client: Client, jarg!(github_user_id): json!(String)) -> re::msg!() {
  if let Some(github_user_id) = db::github_user_id(&github_user_id).await? {
    if let Some(uid) = client.uid().await? {
      let state = m::githubUserSet!(github_user_id, uid);
      if let Ok::<State, _>(s) = state.try_into() {
        let (asset, auth_url) = match s {
          State::UserUsed => (pay_db::asset(uid).await?, "".into()),
          State::Done => {
            let cid = CID::TOPUP as u16;
            let kid = KID::TOPUP::BONUS as _;
            let rid = github_user_id;
            let (asset, set) = tokio::join!(
              pay_db::topup_cent(uid, cid, kid, rid, BONUS),
              m::githubUserSet(github_user_id, uid)
            );
            xerr::log!(set);
            (asset?, "".into())
          }
          _ => (0, github_url()),
        };
        return Ok(api::GithubBonus {
          state: state as _,
          asset,
          auth_url,
          // ..Default::default()
        });
      }
    } else {
      // let mail_li: Vec<(u64, String)> = m::q!(format!("CALL githubUserMail({github_user_id})"));
      return Ok(api::GithubBonus {
        state: State::NeedAuth as _,
        // mail_li: mail_li
        //   .into_iter()
        //   .map(|(id, mail)| api::GithubMail { id, mail })
        //   .collect(),
        ..Default::default()
      });
    }
  }

  let state = State::GithubUserUsed;
  let auth_url = github_url();

  Ok(api::GithubBonus {
    state: state as _,
    asset: 0,
    auth_url,
  })
}

// GET

#[derive(Deserialize)]
pub struct Param {
  code: String,
}

pub async fn get(
  Query(param): Query<Param>,
  request: Request<axum::body::Body>,
) -> re::Result<impl IntoResponse> {
  let headers = request.headers();
  let r = ghapi::access_token(&*GITHUB_STAR_ID, &*GITHUB_STAR_SK, param.code).await?;

  let url;

  #[allow(clippy::never_loop)]
  loop {
    if let Some(token) = ghapi::verify_access_token(r, SCOPE_LI) {
      let user = ghapi::User::new(&token);
      {
        let user = user.clone();
        tokio::spawn(async move {
          xerr::log!(
            user
              .star_user_repo(
                FOLLOW_USER,
                FOLLOW_REPO
                  .iter()
                  .flat_map(|(org, repo_li)| repo_li
                    .iter()
                    .map(move |repo| format!("{org}/{repo}")))
                  .collect::<Vec<_>>(),
              )
              .await
          )
        });
      }
      if let Ok((info, mut email_li)) = xerr::ok!(try_join!(user.info(), user.emails())) {
        //   UserInfo { login: "i18nsite", id: 145643935, followers: 1, following: 8, created_at: "2023-09-21T05:54:58Z", updated_at: "2024-08-02T07:04:51Z", twitter_username: None, name: Some("i18n.site"), company: None }
        // [UserEmail { email: "i18n.site@gmail.com", primary: true }]
        for i in &mut email_li {
          i.email = xmail::norm(i.email.clone());
        }

        let id = info.id;

        m::githubNew!(
          id,
          info.login,
          &token[4..], // 去掉开头的 gho_
          info.followers,
          sonic_rs::to_string(
            &email_li
              .into_iter()
              .map(|i| (i.email, i.primary))
              .collect::<Vec<_>>()
          )?,
          info.twitter_username.unwrap_or_default(),
          info.company.unwrap_or_default(),
          info.name.unwrap_or_default()
        );

        if let Some(host) = headers.get("host") {
          let host = xtld::host_port_tld(host.to_str()?);
          let id = ub64::u64_b64(id);
          url = format!("//{host}/github?{id}");
          break;
        }
      }
    }
    url = github_url();
    break;
  }
  Ok(Redirect::permanent(&url))
}
