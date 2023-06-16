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
    let passphrase = "fake_passwarod".to_string();

    let exchange_client = OkCoinClient::new(api_key, passphrase, url_base, secret);

    let service = Service::new(timeout, threshold, address, exchange_client);
    service.run().await?;
    Ok(())

    // loop {
        // if let Err(err) = service.run() {
        //     return Err(err);
        // }
    // }
}
