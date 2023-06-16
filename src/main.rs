use std::time::Duration;

use constants::*;

use crate::balance_withdrawal::*;

mod balance_withdrawal;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timeout = Duration::from_secs(3);
    let threshold = 100.0;
    let address = RECIPIENT_ADDR_1.to_string();
    let url_base = URL_BASE.to_string();
    let api_key = "fake_api".to_string();
    let secret = "fake_secret_key".to_string();
    let passphrase = "fake_password".to_string();

    let okcoin_client = OkCoinClient::new(api_key.clone(), passphrase.clone(), url_base.clone(), secret.clone());

    let service = Service::new(timeout, threshold, address.clone(), okcoin_client.clone());

    // let current_balance = ExchangeClient::get_balance(&okcoin_client).await?;
    let current_balance = OkCoinClient::get_balance(&okcoin_client).await?;
    // let current_balance = service.exchange_client.get_balance().await?;

    let withdraw = ExchangeClient::withdraw(&okcoin_client, current_balance, address).await?;
    dbg!(withdraw);

    println!("\nWe got the balance: {current_balance}\n");

    service.run().await?;
    
    Ok(())
}
