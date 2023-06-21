mod client;
mod service;

use std::time::Duration;

use client::OkCoinClient;
use service::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let timeout = Duration::from_secs(3);
    let threshold = 100.0;
    let address_1 = "RECIPIENT_ADDR_1".to_string();
    let address_2 = "RECIPIENT_ADDR_2".to_string();

    let api_key = "fake_api".to_string();
    let secret = "fake_secret_key".to_string();
    let passphrase = "fake_password".to_string();

    let okcoin_client = OkCoinClient::new(api_key, passphrase, secret);

    let service = Service::new(timeout, threshold, address_1.clone(), address_2.clone(), okcoin_client);

    service.run().await?;
    
    Ok(())
}
