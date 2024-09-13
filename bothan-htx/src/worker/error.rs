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

#[derive(Error, Debug)]
#[error(transparent)]
pub struct BuildError(#[from] api::ConnectionError);
