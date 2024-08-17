use aok::Result;
use m::mysql_async::Conn;

use crate::{api::Token, db};

pub async fn new_with_conn(conn: &mut Conn, uid: u64, name: impl Into<String>) -> Result<Token> {
  let name = name.into();
  let ts = sts::sec();

  m::e!(
      &mut (*conn);
      "INSERT INTO token(uid,name,ts) VALUES(?,?,?)",
      uid,
      &name,
      ts
  );

  let token_id = conn.last_insert_id().unwrap();
  db::sk::set(token_id, ts).await?;

  Ok(Token {
    id: token_id,
    sk: db::sk::b64(uid, token_id, ts),
    name,
    enable: true,
  })
}

pub async fn new(uid: u64, name: impl Into<String>) -> Result<Token> {
  let mut conn = m::conn!();
  new_with_conn(&mut conn, uid, name).await
}
