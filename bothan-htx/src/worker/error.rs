use crate::api;
use bothan_core::store;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WorkerError {
    #[error("invalid price: {0}")]
    InvalidPrice(f64),

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
