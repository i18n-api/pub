use client::Uid;
use jarg::{jarg, json};
use pay_db::exchange;

use crate::{
  api,
  bind::bind,
  db::stripe::{brand_name_li, payment_method_li, setup_intent},
};

pub async fn post(Uid(uid): Uid, jarg!(synced): json!(u8)) -> re::msg!() {
  if synced == 0 {
    setup_intent::pull(uid).await?;
  };

  let li = payment_method_li(uid).await?;

  let setup = if li.is_empty() {
    Some(bind(uid).await?)
  } else {
    None
  };

  let rate = exchange::BASE; //crate::db::exchange::eur();
  let brand_li = brand_name_li(li.iter().map(|i| i.brand_id)).await?;
  // let (brand_li, rate) = trt::join!(brand_name_li(li.iter().map(|i| i.brand_id)), rate);

  Ok(api::Topup {
    card_li: li
      .into_iter()
      .map(|i| api::Card {
        id: i.id,
        brand_id: i.brand_id as _,
        exp: i.exp as _,
        status: i.status as _,
        name: i.name,
      })
      .collect(),
    brand_li: brand_li
      .into_iter()
      .map(|i| api::Brand {
        id: i.0 as _,
        name: i.1,
      })
      .collect(),
    setup,
    rate,
  })
}
