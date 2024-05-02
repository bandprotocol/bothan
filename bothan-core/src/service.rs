use crate::types::PriceData;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pending result")]
    PendingResult,

    #[error("invalid symbol")]
    InvalidSymbol,
}

/// Type alias for a service result, which is either a valid result or an error.
pub type ServiceResult<T> = Result<T, Error>;

/// The universal trait for all services that provide price data.
#[async_trait::async_trait]
pub trait Service: Send + Sync + 'static {
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>>;
}
