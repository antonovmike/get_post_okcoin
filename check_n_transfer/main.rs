use reqwest::Client;
use serde_json::Value;

const YOUR_ACCOUNT_ADDRESS: &str = "your_account_address";
const RECIPIENT_ADDRESS_1: &str = "recipient_address_1";
// const RECIPIENT_ADDRESS_2: &str = "recipient_address_2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");

    let amount: i64 = 1000;

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

    if response["balance"] != "null" {
        let current_balance = 1001;
        if current_balance >= amount {
            withdrawal(api_key, api_secret, amount.to_string()).await?;
        }
    }

    Ok(())
}

async fn withdrawal(api_key: String, api_secret: String, amount: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://www.okcoin.com/api/v5/private/transaction/withdraw?api_key={}&api_secret={}&currency=STX&amount={}&address={}", 
        api_key, api_secret, amount, RECIPIENT_ADDRESS_1
    );

    // let response: Value = client.get(&url).send().await?.json().await?;
    let response = client.get(&url).send().await?;

    println!("{:#?}", response);

    Ok(())
}