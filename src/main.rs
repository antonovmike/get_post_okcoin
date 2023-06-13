use std::thread;
use std::time::Duration;
use reqwest::Client;

use constants::*;

mod balance_withdrawal;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut account_counter = 2;

    loop {
        let ok_click = balance_withdrawal::OkClick {
            access_key: dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found"),
            passhphrase: dotenv::var("OKCOIN_PASS_PHRASE").expect("OKCOIN_PASS_PHRASE not found"),
            base_url: URL_BASE.to_string(),
            http_client: Client::new(),
        };
        let _current_balance = balance_withdrawal::XClient::get_balance(&ok_click);
        let current_balance = 0.1;

        let address = balance_withdrawal::Address {
            recipient_addr_1: RECIPIENT_ADDR_1.to_string(),
            recipient_addr_2: RECIPIENT_ADDR_2.to_string(),
        };

        if current_balance >= AMOUNT {
            if account_counter == 2 {
                balance_withdrawal::XClient::withdrawal(&ok_click, current_balance, address.recipient_addr_1).await?;
                account_counter = 1
            } else {
                balance_withdrawal::XClient::withdrawal(&ok_click, current_balance, address.recipient_addr_2).await?;
                account_counter = 2
            }
        }

        thread::sleep(Duration::from_secs(3));
    }

    #[allow(unused)]
    Ok(())
}
