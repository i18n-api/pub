use intbin::u64_bin;
use jarg::{jarg, json};
use r::{fred::interfaces::KeysInterface, R};
use t3::HeaderMap;
use xbin::concat;

use crate::{db::code, i18n, throw, K};

pub async fn post(
  header: HeaderMap,
  jarg!(uid, new_mail, old_code, new_code): json!(u64, String, Option<String>, String),
) -> re::msg!() {
  let old_mail: String = m::authUidMail!(uid);
  if !old_mail.is_empty() {
    #[allow(clippy::never_loop)]
    loop {
      if let Some(old_code) = old_code {
        if code::verify(
          i18n::MODIFY_MAIL,
          &old_mail,
          &new_mail,
          old_code.trim().to_owned(),
        ) {
          break;
        }
      }
      throw!(header, now, CODE, INVALID)
    }
  }

  if !code::verify(
    i18n::MODIFY_MAIL,
    &new_mail,
    &old_mail,
    new_code.trim().to_owned(),
  ) {
    throw!(header, mail, CODE, INVALID)
  }

  let code = m::authUidMailUpdate!(uid, &new_mail);
  if code < 0 {
    throw!(header, mail, CODE, MAIL_USED)
  }
  let uid_bin = u64_bin(uid);
  () = R
    .set(concat!(K::UID_ACCOUNT, uid_bin), new_mail, None, None, true)
    .await?;
  Ok(())
}
