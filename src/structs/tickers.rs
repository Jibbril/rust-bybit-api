use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct TickersApiResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,

    #[serde(rename = "retMsg")]
    ret_msg: String,

    pub result: MarketResult,

    #[serde(rename = "retExtInfo")]
    ret_ext_info: Value,

    time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketResult {
    pub category: String,
    pub list: Vec<MarketItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketItem {
    symbol: String,

    #[serde(rename = "bid1Price")]
    bid1_price: String,

    #[serde(rename = "bid1Size")]
    bid1_size: String,

    #[serde(rename = "ask1Price")]
    ask1_price: String,

    #[serde(rename = "ask1Size")]
    ask1_size: String,

    #[serde(rename = "lastPrice")]
    pub last_price: String,

    #[serde(rename = "prevPrice24h")]
    prev_price_24h: String,

    #[serde(rename = "price24hPcnt")]
    price_24h_pcnt: String,

    #[serde(rename = "highPrice24h")]
    high_price_24h: String,

    #[serde(rename = "lowPrice24h")]
    low_price_24h: String,

    #[serde(rename = "turnover24h")]
    turnover_24h: String,

    #[serde(rename = "volume24h")]
    volume_24h: String,

    #[serde(rename = "usdIndexPrice")]
    usd_index_price: String,
}
