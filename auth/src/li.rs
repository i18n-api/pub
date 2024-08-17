use client::Client;
use r::{
  fred::interfaces::{KeysInterface, SortedSetsInterface},
  R,
};
use xbin::concat;

use crate::{api, K};

pub async fn post(client: Client) -> re::msg!() {
  let id_score_li: Vec<(Vec<u8>, i64)> = R
    .zrange(K::client_uid(&client.bin()), 0, -1, None, false, None, true)
    .await?;

  let mut li = Vec::with_capacity(id_score_li.len());
  if !id_score_li.is_empty() {
    let uid_bin_li: Vec<_> = id_score_li.iter().map(|i| &i.0[..]).collect();

    let p = R.pipeline();
    () = p
      .mget(
        uid_bin_li
          .iter()
          .map(|b| concat([K::UID_ACCOUNT, b]).into_boxed_slice())
          .collect::<Vec<_>>(),
      )
      .await?;
    () = p
      .mget(
        uid_bin_li
          .iter()
          .map(|b| concat([user::K::NAME, b]).into_boxed_slice())
          .collect::<Vec<_>>(),
      )
      .await?;
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
