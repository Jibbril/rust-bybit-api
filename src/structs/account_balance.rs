use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceApiResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,

    #[serde(rename = "retMsg")]
    ret_msg: String,

    pub result: AccountBalanceApiResult,

    #[serde(rename = "retExtInfo")]
    ret_ext_info: HashMap<String, serde_json::Value>, // Assuming retExtInfo is a map with dynamic keys
    
    time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalanceApiResult {
    pub list: Vec<AccountInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountInfo {
    #[serde(rename = "totalEquity")]
    total_equity: String,

    #[serde(rename = "accountIMRate")]
    account_im_rate: String,

    #[serde(rename = "totalMarginBalance")]
    total_margin_balance: String,

    #[serde(rename = "totalInitialMargin")]
    total_initial_margin: String,

    #[serde(rename = "accountType")]
    account_type: String,

    #[serde(rename = "totalAvailableBalance")]
    pub total_available_balance: String,

    #[serde(rename = "accountMMRate")]
    account_mm_rate: String,

    #[serde(rename = "totalPerpUPL")]
    total_perp_upl: String,

    #[serde(rename = "totalWalletBalance")]
    total_wallet_balance: String,

    #[serde(rename = "accountLTV")]
    account_ltv: String,

    #[serde(rename = "totalMaintenanceMargin")]
    total_maintenance_margin: String,

    coin: Vec<CoinInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinInfo {
    #[serde(rename = "availableToBorrow")]
    available_to_borrow: String,

    bonus: String,

    #[serde(rename = "accruedInterest")]
    accrued_interest: String,

    #[serde(rename = "availableToWithdraw")]
    available_to_withdraw: String,

    #[serde(rename = "totalOrderIM")]
    total_order_im: String,

    equity: String,

    #[serde(rename = "totalPositionMM")]
    total_position_mm: String,

    #[serde(rename = "usdValue")]
    usd_value: String,

    #[serde(rename = "spotHedgingQty")]
    spot_hedging_qty: String,

    #[serde(rename = "unrealisedPnl")]
    unrealised_pnl: String,

    #[serde(rename = "collateralSwitch")]
    collateral_switch: bool,

    #[serde(rename = "borrowAmount")]
    borrow_amount: String,

    #[serde(rename = "totalPositionIM")]
    total_position_im: String,

    #[serde(rename = "walletBalance")]
    wallet_balance: String,

    #[serde(rename = "cumRealisedPnl")]
    cum_realised_pnl: String,

    locked: String,

    #[serde(rename = "marginCollateral")]
    margin_collateral: bool,

    coin: String,
}
