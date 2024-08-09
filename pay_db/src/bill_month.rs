use std::collections::HashSet;

use aok::OK;
use r::{fred::interfaces::HashesInterface, R};

use super::K;

async fn _bill_month(uid: u64, mut li: Vec<i32>) -> aok::Result<Vec<i32>> {
  let now = sts::now_month() - 1;
  let last = if li.is_empty() { 1 } else { li.pop().unwrap() };

  if last != now {
    let last = last + 1;
    let range = sts::month_day(last);
    let start = range.start;
    let end = if last == now {
      range.end
    } else {
      sts::month_day(now).end
    };
    let day_li: Vec<i32> = m::q!(format!(
      "SELECT DISTINCT day FROM payBill WHERE uid={uid} AND day>={start} AND day<{end}"
    ));
    let mut month_set = HashSet::new();
    for i in day_li {
      month_set.insert(sts::day_month(i));
    }
    let mut mli: Vec<_> = month_set.into_iter().collect();
    mli.sort();
    li.append(&mut mli);
    let mut li = li.clone();
    trt::bg(async move {
      li.push(now);
      let uid_bin = intbin::u64_bin(uid);
      let li_bin = vb::diffe(li.into_iter().map(|i| i as u64).collect::<Vec<_>>());
      () = R.hset(K::PAY_MONTH, (&uid_bin[..], &li_bin[..])).await?;
      OK
    });
  }
  Ok(li)
}

pub async fn bill_month(uid: u64) -> aok::Result<Vec<i32>> {
  // "SELECT DISTINCT day FROM payBill WHERE uid= AND day>=end AND day<"
  let uid_bin = intbin::u64_bin(uid);
  let month_li: Vec<u8> = R.hget(K::PAY_MONTH, &uid_bin[..]).await?;
  _bill_month(
    uid,
    if month_li.is_empty() {
      vec![]
    } else {
      vb::diffd(month_li)?.into_iter().map(|i| i as _).collect()
    },
  )
  .await
}
