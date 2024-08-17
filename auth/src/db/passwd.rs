use aok::Result;

#[derive(Debug, Clone)]
pub struct Salt {
  ts: u64,
  uid: u64,
}

impl Salt {
  pub fn as_bytes(&self) -> Vec<u8> {
    vb::e([self.ts, self.uid])
  }
}

pub async fn set(uid: u64, passwd: impl AsRef<str>) -> Result<()> {
  let passwd = passwd.as_ref().as_bytes();
  let ts = sts::sec();
  let salt = Salt { ts, uid };
  let hash = passwd::hash(&salt.as_bytes(), passwd);
  m::authPasswdSet!(uid, hash, ts);
  Ok(())
}

pub async fn exist(uid: u64) -> Result<Option<(Vec<u8>, u64)>> {
  Ok(m::q01!("SELECT hash,ts FROM authPasswd WHERE id=?", uid))
}

pub fn verify_with_hash(uid: u64, passwd: impl AsRef<str>, ts_hash: (Vec<u8>, u64)) -> bool {
  let passwd = passwd.as_ref();
  let salt = Salt { ts: ts_hash.1, uid };
  let passwd = passwd.as_bytes();
  passwd::verify(&salt.as_bytes(), passwd, &ts_hash.0)
}

pub async fn verify(uid: u64, passwd: impl AsRef<str>) -> Result<bool> {
  let pre = exist(uid).await?;
  if let Some(pre) = pre {
    return Ok(verify_with_hash(uid, passwd, pre));
  }
  Ok(false)
}
