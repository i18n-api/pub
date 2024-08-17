use client::Uid;

use crate::api;

genv::s!(STRIPE_PK);

pub async fn bind(uid: u64) -> m::Result<api::Setup> {
  Ok(api::Setup {
    mail: m::authUidMail!(uid),
    stripe_pk: STRIPE_PK.to_owned(),
  })
}

pub async fn post(Uid(uid): Uid) -> re::msg!() {
  Ok::<api::Setup, _>(bind(uid).await?)
}
