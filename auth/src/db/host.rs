use r::fred::interfaces::SortedSetsInterface;
use xstr::reverse;

use crate::K;

pub async fn id(host: impl AsRef<str>) -> re::Result<u64> {
  tp::host_is_bind(r::R.zscore(K::HOST_ID, reverse(host.as_ref())).await?)
}

pub async fn id_by_header(header: &t3::HeaderMap) -> re::Result<u64> {
  Ok(by_header(header).await?.1)
}

pub async fn by_header(header: &t3::HeaderMap) -> re::Result<(String, u64)> {
  let host = t3::origin_tld(header)?;
  let id = id(&host).await?;
  Ok((host, id))
}
