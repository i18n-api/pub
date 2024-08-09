use r::fred::error::RedisError;

use crate::api::User;

pub fn from_user(id: u64, user: user::User) -> User {
  crate::api::User {
    id,
    ver: user.ver,
    lang: user.lang,
    name: user.name,
  }
}

pub async fn by_id(id: u64) -> Result<User, RedisError> {
  let uid_bin = intbin::u64_bin(id);
  let user = user::by_id_bin(&uid_bin).await?;
  Ok(from_user(id, user))
}

pub async fn by_id_bin(id_bin: impl AsRef<[u8]>) -> Result<User, RedisError> {
  let id_bin = id_bin.as_ref();
  let id = intbin::bin_u64(id_bin);
  let user = user::by_id_bin(&id_bin).await?;
  Ok(from_user(id, user))
}
