CREATE FUNCTION `authUidMailNew`(`hostId` BIGINT UNSIGNED,`mail` VARBINARY(255)) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE mailId BIGINT UNSIGNED;
  DECLARE uid BIGINT UNSIGNED;
  SELECT authMailNew(mail) INTO mailId;
  SELECT id INTO uid FROM authUidMail t WHERE t.authMailId=mailId AND t.hostId=hostId;
  IF uid IS NULL THEN
    SET uid = authUidMailIdNew(hostId,mailId);
  END IF;
  RETURN uid;
END ;;