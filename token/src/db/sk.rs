use intbin::u64_bin;
use r::fred::interfaces::{HashesInterface, RedisResult};
use sha2::{Digest, Sha256};
use uid_by_token::{BEGIN_TS, HASH_LEN, TOKEN_SK};

use crate::K;

pub fn bin(uid: u64, token_id: u64, ts: u64) -> Vec<u8> {
  let bin = &vb::e([uid, token_id, ts - BEGIN_TS])[..];
  let mut hasher = Sha256::new();
  hasher.update(bin);
  hasher.update(&TOKEN_SK[..]);
  let hash = hasher.finalize();
  [&hash[..HASH_LEN], bin].concat()
}

pub fn b64(uid: u64, token_id: u64, ts: u64) -> String {
  let vid = bin(uid, token_id, ts);
  ub64::b64e(vid)
}

pub async fn set_bin(token_id_bin: &[u8], ts: u64) -> RedisResult<()> {
  r::R
    .hset(K::TOKEN, (token_id_bin, u64_bin(ts - BEGIN_TS)))
    .await
}

pub async fn set(token_id: u64, ts: u64) -> RedisResult<()> {
  set_bin(&u64_bin(token_id)[..], ts).await
}
