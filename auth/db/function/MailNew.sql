CREATE FUNCTION `authMailNew`(mail VARBINARY(255)) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE h BIGINT UNSIGNED;
  DECLARE mid BIGINT UNSIGNED;
  CALL splitMail(mail,@u,@h);
  SELECT authMailHostId(@h) INTO h;
  SELECT id INTO mid FROM authMail WHERE usr=@u AND hostId=h;
  IF mid IS NULL THEN
    INSERT INTO authMail (usr,hostId) VALUES (@u,h);
  RETURN LAST_INSERT_ID();
  END IF;
return mid;
END ;;