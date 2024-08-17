CREATE FUNCTION `authOsVerId`(os VARBINARY(255),major SMALLINT UNSIGNED,minor SMALLINT UNSIGNED) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE authOsId BIGINT UNSIGNED;
  DECLARE r BIGINT UNSIGNED;
  SET authOsId=authOsId(os);
  SELECT id INTO r FROM authOsVer t WHERE t.authOsId=authOsId AND t.major=major AND t.minor=minor;
  IF r IS NULL THEN
    INSERT INTO authOsVer (authOsId,major,minor) VALUES (authOsId,major,minor);
  END IF;
RETURN LAST_INSERT_ID();
END ;;