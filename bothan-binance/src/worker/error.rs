use crate::api;
use bothan_core::store;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("failed to connect: {0}")]
    FailedToConnect(#[from] api::ConnectionError),

    #[error("store error: {0}")]
    StoreError(#[from] store::error::Error),
}
