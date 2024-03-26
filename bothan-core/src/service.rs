use crate::types::PriceData;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error: {0}")]
    Unknown(String),

    #[error("pending result")]
    Pending,

    #[error("invalid symbol")]
    InvalidSymbol,

    #[error("websocket error: {0}")]
    Websocket(String),

    #[error("rest error: {0}")]
    Rest(String),
}

pub type ServiceResult<T> = Result<T, Error>;

#[async_trait::async_trait]
pub trait Service: Send + Sync + 'static {
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>>;
}
