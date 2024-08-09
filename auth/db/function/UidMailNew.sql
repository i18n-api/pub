CREATE FUNCTION `authUidMailNew`(`hostId` BIGINT UNSIGNED,`mail` VARBINARY(255)) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE mailId BIGINT UNSIGNED;
  DECLARE uid BIGINT UNSIGNED;
  DECLARE now BIGINT UNSIGNED;
  SET now=unix_timestamp();
  SELECT authMailNew(mail) INTO mailId;
  SELECT id INTO uid FROM authUidMail t WHERE t.authMailId=mailId AND t.hostId=hostId;
  IF uid IS NULL THEN
    INSERT INTO uidHost (hostId,ts) VALUES (hostId,now);
    SELECT LAST_INSERT_ID() INTO uid;
    INSERT INTO authUidMail (id,hostId,authMailId,ts) VALUES (uid,hostId,mailId,now);
  END IF;
  RETURN uid;
END ;;