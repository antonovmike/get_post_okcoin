use crate::balance_withdrawal::b_and_w;

mod balance_withdrawal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    b_and_w().await?;

    #[allow(unused)]
    Ok(())
}

// fn withdrawal() {}