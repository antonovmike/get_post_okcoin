pub mod okcoin;

use async_trait::async_trait;

#[async_trait]
pub trait ExchangeClient {
    type Err: std::error::Error + Send + Sync + 'static;
    async fn get_balance(&self) -> Result<f64, Self::Err>;
    async fn withdraw(&self, current_balance: f64, address: String) -> Result<(), Self::Err>;
}
