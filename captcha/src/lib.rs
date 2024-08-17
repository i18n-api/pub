#![feature(async_closure)]

mod db;
pub use db::Captcha;
use tokio::time::{sleep, Duration, Instant};

t3::api!();

pub async fn post() -> re::msg!() {
  let start = Instant::now();
  let captcha = db::new().await?;
  let elapsed = start.elapsed();

  // 如果消耗的时间少于0.5秒，进行相应的异步阻塞, 避免快速反复请求
  if elapsed < Duration::from_millis(500) {
    let sleep_duration = Duration::from_millis(500) - elapsed;
    sleep(sleep_duration).await;
  }

  Ok::<api::Captcha, _>(captcha)
}
