CREATE FUNCTION `authBrowserVerId`(browser VARBINARY(255),major SMALLINT UNSIGNED,minor SMALLINT UNSIGNED) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE authBrowserId BIGINT UNSIGNED;
  DECLARE r BIGINT UNSIGNED;
  SET authBrowserId=authBrowserId(browser);
  SELECT id INTO r
  FROM authBrowserVer t
  WHERE t.authBrowserId=authBrowserId AND t.major=major AND t.minor=minor;
  IF r IS NULL THEN
    INSERT INTO authBrowserVer (authBrowserId,major,minor)
    VALUES (authBrowserId,major,minor);
END IF;
RETURN LAST_INSERT_ID();
END ;;