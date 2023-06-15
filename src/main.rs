use std::thread;
use std::time::Duration;

use constants::*;

use crate::balance_withdrawal::*;

mod balance_withdrawal;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timeout = Duration::from_secs(5);
    let threshold = 100.0;
    let service = Service::new(timeout, threshold);

    #[allow(unused)]
    Ok(())
}
