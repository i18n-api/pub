CREATE FUNCTION `authUidMailUpdate`(`uid` BIGINT UNSIGNED,`mail` VARBINARY(255)) RETURNS TINYINT
BEGIN
  DECLARE mailId BIGINT UNSIGNED;
  DECLARE oldUid BIGINT UNSIGNED;
  SELECT authMailNew(mail) INTO mailId;
  SELECT id INTO oldUid FROM authUidMail t WHERE t.id=uid AND t.authMailId=mailId;
  IF oldUid IS NOT NULL THEN
    # new mail used
    RETURN -1;
  END IF;
  UPDATE authUidMail SET authMailId=mailId WHERE id=uid;
  RETURN 0;
END ;;