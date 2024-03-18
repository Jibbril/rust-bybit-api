mod structs;

use anyhow::Result;
use dotenv::dotenv;
use hex::encode;
use hmac::{Hmac,Mac};
use reqwest::{get, Client};
use sha2::Sha256;
use structs::account_balance::AccountInfo;
use crate::structs::{server_time::ServerTimeApiResponse, account_balance::AccountBalanceApiResponse};
use std::{env, collections::HashMap};

type HmacSha256 = Hmac<Sha256>;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server_time = get_server_time().await?;

    println!("Server Time: {:#?}",server_time);

    let account_info = get_account_info(server_time).await?;

    println!("Total Available Balance: {:#?}", account_info.total_available_balance);

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

fn build_url(path: &str) -> String {
    format!("https://api.bybit.com{}", path)
    // format!("https://api-testnet.bybit.com{}", path)
}

async fn get_account_info(server_time: u64) -> Result<AccountInfo> {
    let client = Client::new();
    let params = get_params();
    let base_url = build_url("/v5/account/wallet-balance");
    let url = format!("{}?{}", base_url, params_to_query_str(&params));

    let api_key = env::var("BYBIT_API_KEY")?;
    let recv_window = 5000;
    let signature = generate_hmac_signature(
        server_time,
        api_key.clone(),
        recv_window,
        &params
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

fn get_params() -> HashMap<String, String> {
    let mut params: HashMap<String,String> = HashMap::new();
    params.insert("accountType".to_string(), "UNIFIED".to_string());
    params
}

fn params_to_query_str(params: &HashMap<String,String>) -> String {
    params.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

fn generate_hmac_signature(
    timestamp: u64, 
    api_key: String,
    recv_window: i64,
    params: &HashMap<String,String>
) -> Result<String> {
    let api_secret = env::var("BYBIT_API_SECRET")?;

    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes())?;
    mac.update(timestamp.to_string().as_bytes());
    mac.update(api_key.as_bytes());
    mac.update(recv_window.to_string().as_bytes());
    mac.update(params_to_query_str(params).as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    Ok(encode(code_bytes))
}
