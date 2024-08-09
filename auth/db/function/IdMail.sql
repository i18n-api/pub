CREATE FUNCTION `authIdMail`(`mailId` BIGINT UNSIGNED) RETURNS VARBINARY(255)
BEGIN
return (select CONCAT(authMail.usr,'@',v) from authMail,authMailHost WHERE authMail.id=mailId AND authMailHost.id=hostId);
END ;;