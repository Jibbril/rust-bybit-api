use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketCreateApiResponse{
    #[serde(rename = "retCode")]
    ret_code: i32,

    #[serde(rename = "retMsg")]
    ret_msg: String,

    result: ResultType,

    #[serde(rename = "retExtInfo")]
    ret_ext_info: Value,

    time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ResultType {
    Success(ResultSuccess),
    Failure(ResultFailure),
    Empty(Value), // For handling empty result cases
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultSuccess {
    #[serde(rename = "orderId")]
    order_id: String,

    #[serde(rename = "orderLinkId")]
    order_link_id: String,
}

// Define this struct if the failure has a specific structure you want to capture
// Otherwise, you can use serde_json::Value for generic handling
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultFailure {
    // Define failure-specific fields here
    // Example (if there are any specific fields, otherwise, leave this out):
    // error_code: i32,
}

