//! Bothan core IPFS error types.
//!
//! Defines errors for IPFS client operations.

use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("failed to send request: {0}")]
    RequestFailed(String),

    #[error("ipfs hash does not exist")]
    DoesNotExist,

    #[error("received non-zero status code: {0}")]
    NonZeroStatus(u16),
}
