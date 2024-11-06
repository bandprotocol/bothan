use crate::api;
use bothan_core::store;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WorkerError {
    #[error("value is not a valid decimal: {0}")]
    InvalidDecimal(#[from] rust_decimal::Error),

    #[error("failed to set data to the store: {0}")]
    SetFailed(#[from] store::error::Error),
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("failed to connect: {0}")]
    FailedToConnect(#[from] api::ConnectionError),

    #[error("store error: {0}")]
    StoreError(#[from] store::error::Error),
}
