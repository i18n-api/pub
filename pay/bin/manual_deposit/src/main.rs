use clap::Parser;
use pay_db::{BASE, CID, KID};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[arg(required = true, name = "隶属域名")]
  host: String,
  #[arg(required = true, name = "用户账号")]
  account: String,
  #[arg(required = true, name = "充值金额")]
  amount: f64,
}

#[tokio::main]
async fn main() -> aok::Result<()> {
  let cli = Cli::parse();
  if let Some(host_id) = m::hostId!(&cli.host) {
    if let Some(uid) = m::authMailUid!(host_id, &cli.account) {
      let amount = cli.amount;
      println!(
        r#"用户账户 {}
用户编号 {}
充值金额 {}"#,
        cli.account, uid, amount
      );
      if amount != 0.0 {
        let cid = CID::TOPUP as u16;
        let kid = KID::TOPUP::BONUS as _;
        // let kid = KID::TOPUP::MANUAL as _;
        let rid = 0;
        let amount = (amount * BASE) as u64;
        let remain = pay::db::topup_cent(uid, cid, kid, rid, amount).await? as f64 / BASE;
        println!("当前余额 {}", remain);
      }
      return Ok(());
    } else {
      eprintln!("找不到该邮箱账号");
    }
  } else {
    eprintln!("找不到该域名: {}", cli.host);
  };
  Ok(())
}
