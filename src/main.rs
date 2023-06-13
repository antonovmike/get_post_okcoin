use std::thread;
use std::time::Duration;

use constants::*;

mod balance_withdrawal;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut account_counter = 2;

    // todo

    #[allow(unused)]
    Ok(())
}
