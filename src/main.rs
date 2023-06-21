use std::time::Duration;
// use reqwest::Client;

use constants::*;

use crate::balance_withdrawal::*;

mod balance_withdrawal;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timeout = Duration::from_secs(TIMEOUT);
    let threshold = 100.0;
    let address_1 = RECIPIENT_ADDR_1.to_string();
    let address_2 = RECIPIENT_ADDR_2.to_string();
    let url_base = URL_BASE.to_string();
    let api_key = "fake_api".to_string();
    let secret = "fake_secret_key".to_string();
    let passphrase = "fake_password".to_string();

    let okcoin_client = OkCoinClient::new(api_key, passphrase, url_base, secret);

    let service = Service::new(timeout, threshold, address_1.clone(), address_2.clone(), okcoin_client.clone());

    // let current_balance = ExchangeClient::get_balance(&okcoin_client).await?;
    let current_balance = OkCoinClient::get_balance(&okcoin_client).await?;
    // let current_balance = service.exchange_client.get_balance().await?;

    OkCoinClient::withdraw(&okcoin_client, current_balance, address_1).await?;

    println!("\nWe got the balance: {current_balance}\n");

    service.run().await?;
    
    Ok(())
}
