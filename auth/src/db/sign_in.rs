use crate::db::{host, passwd};

pub enum SignIn {
  Ok(u64),
  AccountNotExist,
  PasswdError,
}

pub async fn sign_in(host: &str, account: &str, passwd: impl AsRef<str>) -> re::Result<SignIn> {
  let host_id = host::id(host).await?;
  if let Some(uid) = m::authHostIdMailUid!(host_id, account) {
    if passwd::verify(uid, passwd).await? {
      return Ok(SignIn::Ok(uid));
    }
    return Ok(SignIn::PasswdError);
  }
  Ok(SignIn::AccountNotExist)
}
