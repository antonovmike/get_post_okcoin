use reqwest::Client;
use serde_json::Value;

const YOUR_ACCOUNT_ADDRESS: &str = "your_account_address";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");

    let client = Client::new();
    let url = format!(
        "https://www.okcoin.com/api/v5/account/balance?asset=STX&address={}&key={}&secret={}",
        YOUR_ACCOUNT_ADDRESS, api_key, api_secret
    );

    let response: Value = client.get(&url).send().await?.json().await?;

    println!(
        "STX data: {}; balance: {}",
        response["data"], response["balance"]
    );

    Ok(())
}
