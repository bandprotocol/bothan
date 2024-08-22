use crate::api;
use bothan_core::store;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ParseError {
    #[error("value is not a valid decimal: {0}")]
    Underflow(#[from] rust_decimal::Error),

    #[error("failed to set data to the store: {0}")]
    StoreError(#[from] store::errors::Error),
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("failed to connect: {0}")]
    FailedToConnect(#[from] api::ConnectionError),

    #[error("failed to get data from the store: {0}")]
    StoreError(#[from] store::errors::Error),
}
