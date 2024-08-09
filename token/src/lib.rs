t3::api!();
mod _mod;
urlmod!();

#[allow(non_snake_case)]
pub mod K;
mod db;

use api::Token;
use client::Uid;

pub async fn post(Uid(uid): Uid) -> re::msg!() {
  let mut conn = m::conn!();

  let li: Vec<(u64, u64, String, bool)> = m::q!(
      &mut conn;
      "SELECT id,ts,name,enable FROM token WHERE uid=? ORDER BY id DESC",
      uid,
  );
  if li.is_empty() {
    return Ok(api::TokenLi {
      li: vec![db::new_with_conn(&mut conn, uid, "").await?],
    });
  }
  Ok(api::TokenLi {
    li: li
      .into_iter()
      .map(|(id, ts, name, enable)| Token {
        id,
        sk: db::sk::b64(uid, id, ts),
        name,
        enable,
      })
      .collect(),
  })
}
