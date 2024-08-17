use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, IntEnum)]
pub enum State {
  Done = 0,
  GithubUserUsed = 1,
  UserUsed = 2,
  NeedAuth = 3,
}

genv::s!(GITHUB_STAR_SK:String,GITHUB_STAR_ID:String);

pub fn github_url() -> String {
  format!(
    "https://github.com/login/oauth/authorize?client_id={}&scope={}",
    &*GITHUB_STAR_ID,
    SCOPE_LI.join(",")
  )
}

pub const SCOPE_LI: &[&str] = &["public_repo", "user:email", "user:follow"];

pub const FOLLOW_USER: &[&str] = &["i18n-site", "i18nsite", "i18n-api", "i18n-cron", "i18n-ops"];

pub const FOLLOW_REPO: &[(&str, &[&str])] = &[
  (
    "i18n-site",
    &[
      "18x",
      "addon",
      "alive",
      "demo.i18n.site",
      "demo.i18n.site.docker",
      "font",
      "ie",
      "lib",
      "md",
      "plugin",
      "rust",
      "site",
      "site.conf",
    ],
  ),
  ("i18n-api", &["pay_webhook", "pub", "srv", "srv.docker"]),
  ("i18n-cron", &["cron"]),
  ("i18n-ops", &["docker", "ops", "os", "ubuntu"]),
];

pub async fn github_user_id(id: &str) -> Result<Option<u64>, mysql_async::Error> {
  let github_user_id = ub64::b64_u64(id);

  if github_user_id != 0 {
    let exist: Option<u64> = m::q01!(format!(
      "SELECT uid FROM githubUser WHERE id={github_user_id} AND uid=0"
    ));

    if exist.is_some() {
      return Ok(Some(github_user_id));
    }
  }

  Ok(None)
}
