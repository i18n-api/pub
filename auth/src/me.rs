use client::Client;
use re::ok;

pub async fn post(client: Client) -> re::msg!() {
  if let Some(id) = client.uid().await? {
    let user: crate::api::User = crate::db::api_user::by_id(id).await?;
    return ok!(user);
  }
  ok!(())
}
// let uid_bin = u64_bin(uid);
// let p = R.pipeline();
// let key = client_uid(&client.bin());
// p.zscore(key, uid_bin).await?;
//
// let (logined,): (Option<u64>, Option<u64>) = p.all().await?;
