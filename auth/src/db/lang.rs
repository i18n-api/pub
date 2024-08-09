use r::fred::interfaces::{HashesInterface, RedisResult};

pub async fn set<C: HashesInterface + Sync>(
  p: &C,
  uid_bin: &[u8],
  lang: impl AsRef<[u8]> + Send,
) -> RedisResult<()> {
  let lang = lang.as_ref();
  () = if lang.is_empty() {
    p.hdel(user::K::LANG, uid_bin).await?
  } else {
    p.hset(user::K::LANG, (uid_bin, lang)).await?
  };
  Ok(())
}
