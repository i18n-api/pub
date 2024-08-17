use r::fred::interfaces::KeysInterface;
use t3::IntoResponse;

pub async fn get() -> re::Result<impl IntoResponse> {
  let r: Option<String> = r::R.get("").await?;
  Ok(r.unwrap_or("Kvrocks".into()))
}
