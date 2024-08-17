// use auth::db::host::by_header;
// use client::Client;
// use jarg::{jarg, json};
// use r::R;
// use t3::HeaderMap;
//
// use crate::db;
//
// pub async fn post(
//   client: Client,
//   header: HeaderMap,
//   jarg!(github_user_id, mail_id): json!(String, u64),
// ) -> re::msg!() {
//   if let Some(github_user_id) = db::github_user_id(&github_user_id).await? {
//     let id: Option<u64> = m::q01!(format!(
//       "SELECT id FROM githubMail WHERE github_user_id={github_user_id} AND auth_mail_id={mail_id}"
//     ));
//
//     let (host, host_id) = by_header(&header).await?;
//
//     let uid: Option<u64> = m::q01!(format!(
//       "SELECT id FROM authUidMail WHERE authMailId={mail_id} AND hostId={host_id}"
//     ));
//     let uid = if let Some(uid) = uid {
//       uid
//     } else {
//       // let uid = m::authUidMailIdNew!(host_id, mail_id);
//       // let name: Option<String> = m::q01!(format!(
//       //   "SELECT name FROM githubName WHERE github_user_id={github_user_id} LIMIT 1 ORDER BY id DESC"
//       // ));
//       // let name = if let Some(name) = name {
//       //   name
//       // } else {
//       //   m::q1!("SELECT usr FROM authMail WHERE id={mail_id}")
//       // };
//       //
//       uid
//     };
//     // let uid_bin = intbin::u64_bin(uid);
//     // client.uid_score(&*R, &uid_bin, sts::sec(), true).await?;
//   }
//   Ok(())
// }
