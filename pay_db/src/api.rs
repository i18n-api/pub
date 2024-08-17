use aok::{Null, OK};
use gcd::Gcd;
use r::fred::interfaces::{FunctionInterface, HashesInterface, SetsInterface};

use super::{CID, K, LUA};
use crate::{asset, cost_cent};

#[derive(Debug, Clone)]
pub struct Api {
  pub threshold: u64,        // 调用次数超过这个值就写一次数据库
  pub threshold_amount: u64, // 上面调用次数对应的扣费金额
  pub kid: u64,
}

impl Api {
  pub async fn pay_on(uid: u64) -> aok::Result<bool> {
    let uid_bin = intbin::u64_bin(uid);
    Ok(r::R.sadd(K::PAY_ON, &uid_bin[..]).await?)
  }

  pub async fn pay_off(uid: u64) -> aok::Result<bool> {
    let uid_bin = intbin::u64_bin(uid);
    Ok(r::R.srem(K::PAY_ON, &uid_bin[..]).await?)
  }

  pub async fn can_pay(uid: u64) -> aok::Result<bool> {
    let uid_bin = intbin::u64_bin(uid);
    Ok(r::R.sismember(K::PAY_ON, &uid_bin[..]).await?)
  }

  pub async fn flush(&self, uid: u64, rid: u64) -> aok::Result<i64> {
    let kid = self.kid;
    let uid_bin = &intbin::u64_bin(uid)[..];
    let key = [K::PAY_N, &intbin::u64_bin(kid)].concat();
    let key = &key[..];
    let n: Option<u64> = r::R.hincrby(key, uid_bin, 0).await?;
    if let Some(n) = n {
      let gcd = self.threshold.gcd(self.threshold_amount);
      let threshold = self.threshold / gcd;
      if n >= threshold {
        let multiple = n / threshold;
        let n = (multiple * threshold) as i64;
        () = r::R.hincrby(key, uid_bin, -n).await?;
        let cost = self.threshold_amount / gcd * multiple;
        return cost_cent(uid, CID::API as _, kid, rid, cost).await;
      }
    }
    asset(uid).await
  }

  pub fn cost(&self, uid: u64, rid: u64, n: u64) {
    let kid = self.kid;
    let threshold = self.threshold;
    let threshold_amount = self.threshold_amount;
    trt::bg(async move { cost_async(uid, kid, rid, n, threshold, threshold_amount).await });
  }
}

pub async fn cost_async(
  uid: u64,
  kid: u64,
  rid: u64,
  n: u64,
  threshold: u64,
  threshold_amount: u64,
) -> Null {
  // 如果开放这个接口,应该要检查这个uid是否隶属于当前host
  let uid_bin = &intbin::u64_bin(uid)[..];
  let key = [K::PAY_N, &intbin::u64_bin(kid)].concat();

  let mut multiple = n / threshold;

  let amount = n % threshold;
  if amount > 0 {
    if let Some::<u64>(m) = r::R
      .fcall(
        LUA::PAY_N,
        &[&key[..]],
        (uid_bin, amount as i64, threshold as i64),
      )
      .await?
    {
      multiple += m;
    }
  }

  if multiple > 0 {
    cost_cent(uid, CID::API as _, kid, rid, threshold_amount * multiple).await?;
  }
  OK
}
