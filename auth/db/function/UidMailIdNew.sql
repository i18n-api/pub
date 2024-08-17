CREATE FUNCTION `authUidMailIdNew`(`hostId` BIGINT UNSIGNED,`mailId` BIGINT UNSIGNED) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE uid BIGINT UNSIGNED;
  DECLARE now BIGINT UNSIGNED;
  SET now=unix_timestamp();
  INSERT INTO uidHost (hostId,ts) VALUES (hostId,now);
  SELECT LAST_INSERT_ID() INTO uid;
  INSERT INTO authUidMail (id,hostId,authMailId,ts) VALUES (uid,hostId,mailId,now);
  RETURN uid;
END ;;