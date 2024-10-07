use bothan_core::store;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::api::errors::ConnectionError;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("missing api key")]
    MissingApiKey,

    #[error("failed to connect: {0}")]
    FailedToConnect(#[from] ConnectionError),

    #[error("internal channel is closed: {0}")]
    InternalChannelError(#[from] SendError<Vec<String>>),

    #[error("store error: {0}")]
    StoreError(#[from] store::error::Error),
}

#[derive(Debug, Error)]
pub(crate) enum WorkerError {
    #[error("value is not a valid decimal: {0}")]
    InvalidDecimal(f64),

    #[error("store error: {0}")]
    StoreError(#[from] store::error::Error),
}
