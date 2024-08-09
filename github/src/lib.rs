#![feature(const_trait_impl)]

use t3::IntoResponse;
t3::api!();

mod _mod;
urlmod!();

#[allow(non_snake_case)]
pub mod K;
mod db;

use client::Client;

pub const SCOPE: &str = "public_repo,user:email,user:follow";
pub const FOLLOW_USER: &[&str] = &["i18n-site", "i18nsite"];

pub const FOLLOW_REPO: &[(&str, &[&str])] = &[
  (
    "i18n-site",
    &[
      "site",
      "18x",
      "demo.i18n.site",
      "demo.i18n.site.docker",
      "lib",
      "font",
      "md",
      "plugin",
      "rust",
      "ie",
      "alive",
      "site.conf",
    ],
  ),
  ("i18n-api", &["pay_webhook", "pub", "srv"]),
];

// access_token: "",
// token_type: "bearer",
// scope: "public_repo,user:email,user:follow",

pub async fn post(client: Client) -> re::msg!() {
  Ok(())
}

pub async fn get() -> re::Result<impl IntoResponse> {
  Ok("")
}
