CREATE TRIGGER `authUidMailUpdate` BEFORE UPDATE ON `authUidMail` FOR EACH ROW BEGIN
  INSERT INTO authUidMailLog (uid,authMailId,hostId,dts) VALUES (OLD.id,OLD.authMailId,OLD.hostId,unix_timestamp()) ON DUPLICATE KEY UPDATE dts=VALUES(dts);
END ;;