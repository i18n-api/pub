CREATE FUNCTION `authUidSignIn`(uid BIGINT UNSIGNED,clientId BIGINT UNSIGNED,ip VARBINARY(16),authUaId BIGINT UNSIGNED) RETURNS TINYINT
BEGIN
  DECLARE sid BIGINT UNSIGNED;
  DECLARE now BIGINT UNSIGNED;
  SET now=unix_timestamp();
  INSERT INTO authUidSignIn (clientId,uid,ip,authUaId,ts)VALUES(clientId,uid,ip,authUaId,now);
  SELECT LAST_INSERT_ID() INTO sid;
  INSERT INTO authUidClient (uid,client,state,lastSignInId,cts)VALUES(uid,clientId,1,sid,now) ON DUPLICATE KEY UPDATE state=1,lastSignInId=sid;
  RETURN NULL;
END ;;