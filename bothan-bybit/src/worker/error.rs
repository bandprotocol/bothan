use crate::api;
use bothan_core::store;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WorkerError {
    #[error("failed to parse message")]
    Parse(#[from] serde_json::Error),

    #[error("value is not a valid decimal: {0}")]
    InvalidDecimal(#[from] rust_decimal::Error),

    #[error("failed to set data to the store: {0}")]
    SetFailed(#[from] store::error::Error),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct BuildError(#[from] api::ConnectionError);
