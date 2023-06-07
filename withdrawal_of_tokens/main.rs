use reqwest::Client;
// use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");
    let recipient = dotenv::var("RECIPIENT_ADDRESS_1").expect("RECIPIENT_ADDRESS_1 not found");

    let amount = "1000";

    let client = Client::new();
    let url = format!(
        "https://www.okcoin.com/api/v5/private/transaction/withdraw?api_key={}&api_secret={}&currency=STX&amount={}&address={}", 
        api_key, api_secret, amount, recipient
    );

    // let response: Value = client.get(&url).send().await?.json().await?;
    let response = client.get(&url).send().await?;

    println!("{:#?}", response);

    Ok(())
}
