CREATE TRIGGER `authPasswordLog` BEFORE UPDATE ON `authPasswd` FOR EACH ROW BEGIN
INSERT IGNORE INTO authPasswdLog (uid,hash,ts,dts) VALUES (OLD.id,OLD.hash,OLD.ts,unix_timestamp());
END ;;