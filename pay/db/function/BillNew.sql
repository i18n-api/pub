CREATE FUNCTION `payBillNew`(`uid` BIGINT UNSIGNED,`cid` SMALLINT UNSIGNED,`kid` BIGINT UNSIGNED,`rid` BIGINT UNSIGNED,`amount` BIGINT,`ts` BIGINT UNSIGNED) RETURNS BIGINT
BEGIN
DECLARE bill_id BIGINT UNSIGNED;
DECLARE day MEDIUMINT UNSIGNED;
DECLARE now_n BIGINT;
DECLARE fee BIGINT;
SET day=ts DIV 86400;
SELECT id INTO bill_id FROM payBill WHERE payBill.uid=uid AND payBill.cid=cid AND payBill.kid=kid AND payBill.rid=rid AND payBill.day=day;
IF bill_id IS NULL THEN
  INSERT INTO payBill (uid,cid,kid,rid,n,day) VALUES (uid,cid,kid,rid,amount,day) ON DUPLICATE KEY UPDATE n=n+amount;
  SET bill_id=LAST_INSERT_ID();
ELSE
  UPDATE payBill SET n=n+amount WHERE id=bill_id;
END IF;
IF cid=0 THEN
  IF kid=1000 THEN
    IF amount<=30 THEN
      SET fee=amount;
    ELSE
      SET fee=ROUND((amount-30)*0.046711153479504275 + 30);
    END IF;
  END IF;
  IF fee IS NOT NULL THEN
    INSERT INTO payBill (uid,cid,kid,rid,n,day) VALUES (uid,2,0,bill_id,-fee,day);
    SET amount=amount-fee;
  END IF;
END IF;
SELECT n INTO now_n FROM payCash WHERE id=uid;
IF now_n IS NULL THEN
  SET now_n=amount;
  INSERT INTO payCash (id,n) VALUES (uid,amount) ON DUPLICATE KEY UPDATE n=n+amount;
ELSE
  IF amount!=0 THEN
    SET now_n=now_n+amount;
    UPDATE payCash SET n=n+amount WHERE id=uid;
  END IF;
END IF;
INSERT INTO payCashLog (ts,n,diff,bill_id) VALUES (ts,now_n,amount,bill_id);
RETURN now_n;
END ;;