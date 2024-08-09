CREATE PROCEDURE `payStripeTopupUpdate`(IN `v` VARBINARY(32),IN `recv` BIGINT UNSIGNED,IN `status` TINYINT UNSIGNED)
BEGIN
  DECLARE pid BIGINT UNSIGNED DEFAULT 0;
  DECLARE old_recv BIGINT UNSIGNED DEFAULT 0;
  DECLARE old_status TINYINT UNSIGNED DEFAULT 0;
  DECLARE updated BIGINT UNSIGNED DEFAULT 0;
  DECLARE uid BIGINT UNSIGNED DEFAULT 0;
  DECLARE rate BIGINT UNSIGNED DEFAULT 0;
  SELECT p.id,p.recv,p.status,p.uid,p.rate INTO pid,old_recv,old_status,uid,rate FROM payStripeTopup AS p WHERE p.v=v;
  IF pid>0 THEN
    IF old_status!=status THEN
      UPDATE payStripeTopup p SET p.status=status,p.recv=recv WHERE p.id=pid AND p.status=old_status AND p.recv=old_recv;
      SET updated=ROW_COUNT();
    END IF;
  END IF;
  SELECT pid,updated,uid,rate;
END ;;