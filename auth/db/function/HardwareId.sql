CREATE FUNCTION `authHardwareId`(w SMALLINT UNSIGNED,h SMALLINT UNSIGNED,pixelRatio TINYINT UNSIGNED,cpu SMALLINT UNSIGNED,gpu VARBINARY(255),arch VARBINARY(255)
) RETURNS BIGINT UNSIGNED
BEGIN
  DECLARE authWidthHeightId BIGINT UNSIGNED;
  DECLARE authGpuId BIGINT UNSIGNED;
  DECLARE authArchId BIGINT UNSIGNED;
  DECLARE r BIGINT UNSIGNED;
  SELECT authWidthHeightId(w,h) INTO authWidthHeightId;
  SELECT authGpuId(gpu) INTO authGpuId;
  SELECT authArchId(arch) INTO authArchId;
  SELECT id INTO r FROM authHardware t
  WHERE t.authWidthHeightId=authWidthHeightId
  AND t.pixelRatio=pixelRatio
  AND t.cpu=cpu
  AND t.authGpuId=authGpuId
  AND t.authArchId=authArchId;
  IF r IS NULL THEN
    INSERT INTO authHardware(authWidthHeightId,pixelRatio,cpu,authGpuId,authArchId)
    VALUES(authWidthHeightId,pixelRatio,cpu,authGpuId,authArchId);
    RETURN LAST_INSERT_ID();
END IF;
RETURN r;
END ;;