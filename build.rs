use aok::{Result, OK};

fn main() -> Result<()> {
  let mut config = prost_build::Config::new();

  config.compile_protos(&["api.proto"], &[std::env!("CARGO_MANIFEST_DIR")])?;

  OK
}
