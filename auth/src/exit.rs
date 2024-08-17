use client::Client;
use jarg::{jarg, json};

use crate::client;

pub async fn post(client: Client, jarg!(uid): json!(u64)) -> re::msg!() {
  client!(client, json, exit, uid)
}
