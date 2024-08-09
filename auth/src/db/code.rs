use sk::sk;

pub fn gen(kind: &[u8], account: impl AsRef<str>, password: impl AsRef<str>, hour: u64) -> String {
  util::hash::token(
    &[
      account.as_ref().as_bytes(),
      password.as_ref().as_bytes(),
      &hour.to_le_bytes()[..],
      sk(),
      kind,
    ]
    .concat(),
  )
}

pub fn verify(
  kind: &[u8],
  account: impl AsRef<str>,
  password: impl AsRef<str>,
  code: String,
) -> bool {
  let code = code.trim();
  let hour = sts::hour();
  let c = gen(kind, &account, &password, hour);
  let mut verifyed = c == code;
  if !verifyed {
    verifyed = gen(kind, &account, &password, hour - 1) == code;
  }
  verifyed
}
