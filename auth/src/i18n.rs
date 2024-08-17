// gen by @3-/i18n-rust ; DON'T EDIT

use r::{
  fred::interfaces::{HashesInterface, RedisResult},
  R,
};

pub const SIGN_UP: &[u8] = b"signUp";

pub const RESET_PASSWORD: &[u8] = b"resetPassword";

pub const MAIL: &[u8] = b"mail";

pub const INVALID: &[u8] = b"invalid";

pub const CODE: &[u8] = b"code";

pub const ACCOUNT_EXIST: &[u8] = b"accountExist";

pub const ACCOUNT_NOT_EXIST: &[u8] = b"accountNotExist";

pub const PASSWORD_ERROR: &[u8] = b"passwordError";

pub const MAIL_USED: &[u8] = b"mailUsed";

pub const MODIFY_MAIL: &[u8] = b"modifyMail";

pub const BAN_MAIL: &[u8] = b"banMail";

pub const VERIFY_MAIL: &[u8] = b"verifyMail";

#[macro_export]
macro_rules! throw {

  ($header:ident,$id:ident,$key:ident) => {{
    $crate::i18n::throw(&$header, stringify!($id), $crate::i18n::$key).await?;
    unreachable!()
  }};

  ($header:ident,$id:ident,$($key:ident),+) => {{
    $crate::i18n::throw_li(&$header, stringify!($id), &[
      $($crate::i18n::$key),+
    ]).await?;
    unreachable!()
  }};

}

::i18n::gen!(auth);