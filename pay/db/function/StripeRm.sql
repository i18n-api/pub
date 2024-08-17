CREATE FUNCTION `payStripeRm`(uid BIGINT UNSIGNED,id BIGINT UNSIGNED) RETURNS TINYINT UNSIGNED
BEGIN
    DECLARE t BIGINT;
    DELETE FROM payStripe WHERE payStripe.id = id AND payStripe.uid = uid;
    SELECT COUNT(1) INTO t FROM payStripe t WHERE t.uid = uid;
    IF t = 0 THEN
        SELECT n INTO t FROM payCash t WHERE t.id=uid;
        IF t > 0 THEN
            RETURN 1;
        ELSE
            RETURN 0;
        END IF;
    ELSE
        RETURN 1;
    END IF;
END ;;