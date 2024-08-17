use aok::Result;
use intbin::{bin_u64, u64_bin};
use r::{fred::interfaces::HashesInterface, R};

use crate::K;

pub async fn ing(uid: u64) -> Result<Vec<(u64, u64)>> {
  let li: Vec<(Vec<u8>, Vec<u8>)> = R
    .hgetall(&[K::PAY_ING, &u64_bin(uid)[..]].concat()[..])
    .await?;

  if li.is_empty() {
    return Ok(vec![]);
  }

  Ok(
    li.into_iter()
      .map(|(k, v)| (bin_u64(&k), bin_u64(&v)))
      .collect(),
  )
}
