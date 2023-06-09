use std::thread;
use std::time::Duration;

use crate::balance_withdrawal::b_and_w;

use balance_withdrawal::withdrawal;
use constants::*;

mod balance_withdrawal;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut account_counter = 2;

    loop {
        let current_balance = b_and_w().await?;

        if current_balance >= AMOUNT {
            if account_counter == 2 {
                withdrawal(current_balance, RECIPIENT_ADDR_1).await?;
                account_counter = 1
            } else {
                withdrawal(current_balance, RECIPIENT_ADDR_2).await?;
                account_counter = 2
            }
        }

        thread::sleep(Duration::from_secs(3));
    }

    #[allow(unused)]
    Ok(())
}
