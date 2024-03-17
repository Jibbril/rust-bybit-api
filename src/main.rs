mod structs;

use anyhow::Result;
use dotenv::dotenv;
use reqwest::get;
use crate::structs::server_time::ServerTimeApiResponse;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server_time = get_server_time().await?;

    println!("{:#?}",server_time);

    Ok(())
}

async fn get_server_time() -> Result<u64> {
    let url = "https://api-testnet.bybit.com/v5/market/time";

    let res = get(url).await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let resdata: ServerTimeApiResponse = res.json().await?;
            Ok(resdata.time)
        }
        _ => panic!("Unable to fetch server time."),
    }
}
