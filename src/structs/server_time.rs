use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTimeApiResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,
    
    #[serde(rename = "retMsg")]
    ret_msg: String,
    
    result: ServerTimeResultField,
    
    #[serde(rename = "retExtInfo")]
    ret_ext_info: HashMap<String, Value>,
    
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTimeResultField {
    #[serde(rename = "timeSecond")]
    time_second: String,
    
    #[serde(rename = "timeNano")]
    time_nano: String,
}
