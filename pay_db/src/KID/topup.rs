#[allow(non_camel_case_types)]
pub enum TOPUP {
  MANUAL = 0,
  BONUS = 1, // 赠金
  STRIPE = 1000,
  GOOGLE_PAY = 1001,
  ALI = 1002, // 支付宝
  WX = 1003,  // 微信
              // AIRWALLEX = 1004, // 空中云汇
}
