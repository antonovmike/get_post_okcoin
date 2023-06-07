use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");
    let your_acc = dotenv::var("YOUR_ACCOUNT_ADDRESS").expect("YOUR_ACCOUNT_ADDRESS not found");

    let client = Client::new();
    let url = format!(
        "https://www.okcoin.com/api/v5/account/balance?asset=STX&address={}&key={}&secret={}",
        your_acc, api_key, api_secret
    );

    let response: Value = client.get(&url).send().await?.json().await?;

    println!(
        "STX data: {}; balance: {}",
        response["data"], response["balance"]
    );

    Ok(())
}
