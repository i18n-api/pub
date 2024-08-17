CREATE TRIGGER payStripeToRmed AFTER DELETE ON payStripe
FOR EACH ROW
BEGIN
  INSERT INTO payStripeRmed (id,uid,brand_id,name,exp,status,v,ts,kind)
  VALUES (OLD.id,OLD.uid,OLD.brand_id,OLD.name,OLD.exp,OLD.status,OLD.v,OLD.ts,OLD.kind) ON DUPLICATE KEY UPDATE id=OLD.id,uid=OLD.uid,brand_id=OLD.brand_id,name=OLD.name,exp=OLD.exp,status=OLD.status,ts=OLD.ts,kind=OLD.kind;
END ;;