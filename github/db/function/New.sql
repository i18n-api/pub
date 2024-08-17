CREATE FUNCTION `githubNew`(`p_id` BIGINT UNSIGNED,`p_login` VARBINARY(255),`p_token` VARBINARY(255),`p_followers` BIGINT UNSIGNED,`p_mail_li` JSON,`p_x_name` VARCHAR(255),`p_com` VARCHAR(255),`p_name` VARCHAR(255)) RETURNS TINYINT
BEGIN
  IF LENGTH(p_name) > 0 THEN
    INSERT IGNORE INTO githubName(github_user_id,name) VALUES (p_id,p_name);
  END IF;
  IF LENGTH(p_com) > 0 THEN
    INSERT IGNORE INTO githubCom(github_user_id,name) VALUES (p_id,p_com);
  END IF;
  IF LENGTH(p_x_name) > 0 THEN
    INSERT IGNORE INTO githubX(github_user_id,name) VALUES (p_id,p_x_name) ON DUPLICATE KEY UPDATE github_user_id = VALUES(github_user_id);
  END IF;
  INSERT IGNORE INTO githubUserFollowers(github_user_id,n) VALUES (p_id,p_followers);
  IF EXISTS (SELECT 1 FROM githubUser WHERE id=p_id) THEN
    RETURN FALSE;
  END IF;
  INSERT INTO githubUser(id,uid,login,token) VALUES (p_id,0,p_login,p_token);
  INSERT INTO githubMail(github_user_id,auth_mail_id,is_primary)
  SELECT
    p_id,authMailNew(jt.mail),jt.is_primary
  FROM
    JSON_TABLE(p_mail_li,'$[*]' COLUMNS ( mail VARCHAR(255) PATH '$[0]',is_primary TINYINT PATH '$[1]')) AS jt;
  RETURN TRUE;
END ;;