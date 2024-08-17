use r::fred::interfaces::{KeysInterface, RedisResult};
use xbin::concat;

pub async fn set<C: KeysInterface + Sync>(
  p: &C,
  uid_bin: &[u8],
  lang: impl AsRef<[u8]> + Send,
) -> RedisResult<()> {
  let lang = lang.as_ref();
  () = if lang.is_empty() {
    p.del(concat!(user::K::LANG, uid_bin)).await?
  } else {
    p.set(concat!(user::K::LANG, uid_bin), lang, None, None, false)
      .await?
  };
  Ok(())
}
