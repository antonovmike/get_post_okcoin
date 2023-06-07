use reqwest::Client;
// use serde_json::Value;
// use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let api_key = env::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    // let api_secret = env::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found")
    
    let file_api = File::open("account_details/OKCOIN_API_KEY")?;
    let file_secret_api = File::open("account_details/OKCOIN_API_SECRET")?;
    let file_r_addr_1 = File::open("account_details/RECIPIENT_ADDRESS_1")?;
    
    let reader = BufReader::new(file_api);
    let mut api_key = "".to_string();
    for line in reader.lines() {
        api_key = line?;
    }

    let reader = BufReader::new(file_secret_api);
    let mut api_secret = "".to_string();
    for line in reader.lines() {
        api_secret = line?;
    }

    let reader = BufReader::new(file_r_addr_1);
    let mut recipient = "".to_string();
    for line in reader.lines() {
        recipient = line?;
    }

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
