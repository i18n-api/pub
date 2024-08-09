use t3::IntoResponse;

pub async fn get() -> re::Result<impl IntoResponse> {
  let mut conn = m::conn!();
  let r: String = m::q1!(
      &mut conn;
      "SELECT 'Mariadb'"
  );
  Ok(r)
}
