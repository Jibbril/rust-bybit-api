mod structs;

use anyhow::Result;
use dotenv::dotenv;
use hex::encode;
use hmac::{Hmac,Mac};
use reqwest::{get, Client};
use serde_json::{json,Map, to_string, Value};
use sha2::Sha256;
use structs::{account_balance::AccountInfo, tickers::TickersApiResponse};
use crate::structs::{server_time::ServerTimeApiResponse, account_balance::AccountBalanceApiResponse, market_create::MarketCreateApiResponse};
use std::{env, collections::HashMap};

type HmacSha256 = Hmac<Sha256>;

const BASE_CURRENCY: &str = "USDT";
const ORDER_MAX_DECIMALS: i64 = 6;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server_time = get_server_time().await?;
    println!("Server Time: {:#?}",server_time);

    let account_info = get_account_info(server_time).await?;
    println!("Total Available Balance: {:#?}", account_info.total_available_balance);

    let current_price = get_current_price("BTCUSDT").await?;
    println!("Current Price: {:#?}", current_price);

    let buy = false;

    if buy {
        let balance: f64 = account_info.total_available_balance.parse()?;
        market_buy(balance * 0.5).await?;
    } else {
        market_sell_all(&account_info).await?;
    }

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

async fn post_market_order(params: Map<String,Value>) -> Result<()> {
    let client = Client::new();
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

async fn get_current_price(symbol: &str) -> Result<f64> {
    let client = Client::new();

    let mut params: HashMap<String,String> = HashMap::new();
    params.insert("category".to_string(), "spot".to_string());
    params.insert("symbol".to_string(), symbol.to_string());

    let base_url = build_url( "/v5/market/tickers");
    let url = format!("{}?{}", base_url, params_to_query_str(&params));
    let api_key = api_key()?;
    let recv_window = 5000;
    let server_time = get_server_time().await?;
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
            let resdata: TickersApiResponse = res.json().await?;
            resdata
        },
        _ => panic!("Unable to fetch account balance")
    };

    let account_info = response
        .result
        .list
        .first()
        .cloned()
        .expect("Tickers endpoint should return at least one account.");

    let last_price: f64 = account_info.last_price.parse()?;

    Ok(last_price)   
}

async fn market_buy(quantity: f64) -> Result<()> {
    let symbol = "BTCUSDT";
    let rounded_quantity = round(quantity, 2);

    let mut params = Map::new();
    params.insert("category".to_string(), json!("spot"));
    params.insert("symbol".to_string(), json!(symbol));
    params.insert("side".to_string(), json!("Buy"));
    params.insert("orderType".to_string(), json!("Market"));
    params.insert("marketUnit".to_string(), json!("quoteCoin"));
    params.insert("qty".to_string(), json!(rounded_quantity.to_string()));

    println!("params: {:#?}",params);

    Ok(post_market_order(params).await?)
}

async fn market_sell_all(account_info: &AccountInfo) -> Result<()> {
    for coin in account_info.coin.iter() {
        // Skip selling of base currency
        if coin.coin == BASE_CURRENCY { continue; };

        let usd_value: f64 = coin.usd_value.parse()?;

        // Ignore small amounts
        if usd_value < 1.0 { continue; };

        let amount: f64 = coin.wallet_balance.parse()?;
        let amount = floor(amount, ORDER_MAX_DECIMALS);

        // Ignore extremely small quantities of the traded currency
        if amount == 0.0 { continue; };

        let symbol = format!("{}{}", coin.coin, BASE_CURRENCY);

        let mut params = Map::new();
        params.insert("category".to_string(), json!("spot"));
        params.insert("symbol".to_string(), json!(symbol));
        params.insert("side".to_string(), json!("Sell"));
        params.insert("orderType".to_string(), json!("Market"));
        params.insert("qty".to_string(), json!(amount.to_string()));
        params.insert("marketUnit".to_string(), json!("baseCoin"));

        post_market_order(params).await?;
    }
    
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

fn round(x: f64, n: i64) -> f64 {
    let scaling = 10f64.powi(n as i32);
    (x * scaling).round() / scaling
}

fn floor(x: f64, n: i64) -> f64 {
    let scaling = 10f64.powi(n as i32);
    (x * scaling).floor() / scaling
}
