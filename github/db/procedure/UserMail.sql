CREATE PROCEDURE `githubUserMail`(IN `github_user_id` BIGINT UNSIGNED)
BEGIN
  SELECT auth_mail_id AS id,authIdMail(auth_mail_id) AS mail FROM githubMail WHERE github_user_id=github_user_id ORDER BY is_primary DESC;
END ;;