use intbin::u64_bin;
use prost::Message;
use r::{
  fred::{interfaces::KeysInterface, prelude::Expiration},
  R,
};
use rand::Rng;
use t3::{axum::http::header::CONTENT_TYPE, StatusCode};

use crate::api;

pub const CAPTCHA: [u8; 8] = *b"captcha:";

pub const SCALE: u32 = 2;
pub const SIZE: u32 = 420;
const JS_MAX_SAFE_INTEGER: u64 = (1u64 << 53) - 1;

pub struct Captcha();

pub async fn captcha(parts: &mut apart::Parts) -> aok::Result<Captcha> {
  if let Some(Ok(content_type)) = parts.headers.get(CONTENT_TYPE).map(|i| i.to_str()) {
    verify(content_type).await
  } else {
    err().await
  }
}
apart::from_request_parts!(Captcha, captcha);
// pub async fn verify(header: &HeaderMap) -> t2::Result<()> {
//   _verify(
//     header
//       .get("content-type")
//       .map(|v| v.to_str().unwrap_or(""))
//       .unwrap_or(""),
//   )
//   .await
// }
//

async fn err<T>() -> aok::Result<T> {
  apart::err(
    StatusCode::PRECONDITION_FAILED,
    new().await?.encode_to_vec(),
  )?
}

async fn verify(json: &str) -> aok::Result<Captcha> {
  let json: Vec<u64> = sonic_rs::from_str(json)?;
  if json.len() == 7 {
    let key = u64_bin(json[0]);
    let key = [&CAPTCHA[..], &key].concat();
    if let Some(val) = R.get::<Option<Vec<u8>>, _>(&*key).await? {
      trt::bg(async move { R.del::<(), _>(&*key).await });
      if let Ok(val) = vb::d(val) {
        if click_captcha::verify(&val, &json[1..], SCALE as _) {
          return Ok(Captcha());
        }
      }
    }
  }
  err().await?
}

pub async fn new() -> aok::Result<api::Captcha> {
  let g = click_captcha::gen(SIZE * SCALE, SIZE * SCALE)?;

  let mut flag_li = [0; click_captcha::N * 3];

  let mut svg_li = Vec::with_capacity(click_captcha::N);

  for (p, i) in g.1.into_iter().enumerate() {
    svg_li.push(click_captcha::FLAG[i.pos].to_string());
    let p = p * 3;
    flag_li[p] = i.x as _;
    flag_li[p + 1] = i.y as _;
    flag_li[p + 2] = i.size as _;
  }

  let flag_li = vb::e(flag_li);
  let mut key_id;
  loop {
    key_id = rand::thread_rng().gen_range(1..=JS_MAX_SAFE_INTEGER);
    let key = u64_bin(key_id);
    let key = &*[&CAPTCHA[..], &key].concat();
    if !R.exists::<bool, _>(key).await? {
      () = R
        .set(key, &flag_li[..], Some(Expiration::EX(300)), None, false)
        .await?;
      break;
    }
  }
  Ok(api::Captcha {
    id: key_id,
    img: g.0.into(),
    svg_li,
  })
  // r.push(g.0);
  // r.push(key_id);
  // Ok(r)
}
