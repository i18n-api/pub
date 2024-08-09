CREATE PROCEDURE `payBill`(IN `uid` BIGINT UNSIGNED,IN `begin` MEDIUMINT,IN `end` MEDIUMINT)
BEGIN
  SELECT cid,kid,rid,day,n FROM payBill WHERE payBill.uid=uid AND day>=begin AND day<end;
END ;;