use client::Client;

pub async fn post(client: Client) -> re::msg!() {
  Ok(client.exit_all().await?)
}
