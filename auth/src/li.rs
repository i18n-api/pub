use client::Client;
use r::{
  fred::interfaces::{HashesInterface, SortedSetsInterface},
  R,
};

use crate::{api, K};

pub async fn post(client: Client) -> re::msg!() {
  let id_score_li: Vec<(Vec<u8>, i64)> = R
    .zrange(K::client_uid(&client.bin()), 0, -1, None, false, None, true)
    .await?;

  let mut li = Vec::with_capacity(id_score_li.len());
  if !id_score_li.is_empty() {
    let uid_bin_li: Vec<_> = id_score_li.iter().map(|i| &i.0[..]).collect();

    let p = R.pipeline();
    () = p.hmget(K::UID_ACCOUNT, uid_bin_li.clone()).await?;
    () = p.hmget(user::K::NAME, uid_bin_li.clone()).await?;
    let (account_li, name_li): (Vec<String>, Vec<String>) = p.all().await?;

    for (p, id) in uid_bin_li.into_iter().map(intbin::bin_u64).enumerate() {
      li.push(api::UserState {
        id,
        name: name_li[p].clone(),
        account: account_li[p].clone(),
        exit: id_score_li[p].1 < 0,
      })
    }
  }
  Ok(api::UserStateLi { li })
}
