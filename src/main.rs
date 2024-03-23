mod structs;

use anyhow::Result;
use dotenv::dotenv;
use hex::encode;
use hmac::{Hmac,Mac};
use reqwest::{get, Client};
use serde_json::{json,Map, to_string};
use sha2::Sha256;
use structs::account_balance::AccountInfo;
use crate::structs::{server_time::ServerTimeApiResponse, account_balance::AccountBalanceApiResponse, market_create::MarketCreateApiResponse};
use std::{env, collections::HashMap};

type HmacSha256 = Hmac<Sha256>;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server_time = get_server_time().await?;

    println!("Server Time: {:#?}",server_time);

    let account_info = get_account_info(server_time).await?;

    println!("Total Available Balance: {:#?}", account_info.total_available_balance);

    let balance: f64 = account_info.total_available_balance.parse()?;
    market_buy(balance * 0.1).await?;

    Ok(())
}

async fn get_server_time() -> Result<u64> {
    let url = build_url("/v5/market/time");

    let res = get(url).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let resdata: ServerTimeApiResponse = res.json().await?;
            Ok(resdata.time)
        }
        _ => panic!("Unable to fetch server time."),
    }
}

async fn get_account_info(server_time: u64) -> Result<AccountInfo> {
    let client = Client::new();

    let mut params: HashMap<String,String> = HashMap::new();
    params.insert("accountType".to_string(), "UNIFIED".to_string());

    let base_url = build_url("/v5/account/wallet-balance");
    let url = format!("{}?{}", base_url, params_to_query_str(&params));
    let api_key = api_key()?;
    let recv_window = 5000;
    let signature = generate_hmac_signature(
        server_time,
        &api_key,
        recv_window,
        params_to_query_str(&params)
    )?;

    let res = client.get(url)
        .header("X-BAPI-SIGN", signature)
        .header("X-BAPI-API-KEY", api_key)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", server_time)
        .header("X-BAPI-RECV-WINDOW", recv_window)
        .send()
        .await?;

    let response = match res.status() {
        reqwest::StatusCode::OK => {
            let resdata: AccountBalanceApiResponse = res.json().await?;
            resdata
        },
        _ => panic!("Unable to fetch account balance")
    };

    let account_info = response
        .result
        .list
        .first()
        .cloned()
        .expect("Should return at least one account.");

    Ok(account_info)
}

async fn market_buy(amount: f64) -> Result<()> {
    let rounded_amount = (amount * 100.0).round() / 100.0;
    let client = Client::new();

    let mut params = Map::new();
    params.insert("category".to_string(), json!("spot"));
    params.insert("symbol".to_string(), json!("BTCUSDT"));
    params.insert("side".to_string(), json!("Buy"));
    params.insert("orderType".to_string(), json!("Market"));
    params.insert("qty".to_string(), json!(rounded_amount.to_string()));
    params.insert("marketUnit".to_string(), json!("quoteCoin"));
    // params.insert("orderLinkId".to_string(), json!("my-second-nice-trade"));

    let timestamp = get_server_time().await?;
    let recv_window = 5000;
    let api_key = &api_key()?;

    let signature = generate_hmac_signature(
        timestamp, 
        &api_key, 
        recv_window, 
        to_string(&params)?
    )?;

    let res = client.post(build_url("/v5/order/create"))
        .json(&params)
        .header("X-BAPI-SIGN", signature)
        .header("X-BAPI-API-KEY", api_key)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", timestamp)
        .header("X-BAPI-RECV-WINDOW", recv_window)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    // println!("Response Text: {:#?}",res.text().await?);

    let response = match res.status() {
        reqwest::StatusCode::OK => {
            let resdata: MarketCreateApiResponse = res.json().await?;
            resdata
        },
        _ => panic!("Unable to perform market buy")
    };

    println!("Create Response: {:#?}", response);

    Ok(())
}

fn build_url(path: &str) -> String {
    format!("https://api.bybit.com{}", path)
}

fn params_to_query_str(params: &HashMap<String,String>) -> String {
    params.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

fn generate_hmac_signature(
    timestamp: u64, 
    api_key: &String,
    recv_window: i64,
    params: String
) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(api_secret()?.as_bytes())?;
    mac.update(timestamp.to_string().as_bytes());
    mac.update(api_key.as_bytes());
    mac.update(recv_window.to_string().as_bytes());
    mac.update(params.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    Ok(encode(code_bytes))
}

fn api_secret() -> Result<String> {
    Ok(env::var("BYBIT_API_SECRET")?)
}

fn api_key() -> Result<String> {
    Ok(env::var("BYBIT_API_KEY")?)
}
