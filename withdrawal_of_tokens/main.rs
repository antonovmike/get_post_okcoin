use reqwest::Client;
// use serde_json::Value;

const RECIPIENT_ADDRESS_1: &str = "recipient_address_1";
const AMOUNT: i64 = 1000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");

    let client = Client::new();
    let url = format!(
        "https://www.okcoin.com/api/v5/private/transaction/withdraw?api_key={}&api_secret={}&currency=STX&amount={}&address={}", 
        api_key, api_secret, AMOUNT, RECIPIENT_ADDRESS_1
    );

    // let response: Value = client.get(&url).send().await?.json().await?;
    let response = client.get(&url).send().await?;

    println!("{:#?}", response);

    Ok(())
}
