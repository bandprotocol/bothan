use thiserror::Error;

use crate::store::error::Error as StoreError;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum SetRegistryError {
    #[error("Failed to set registry: {0}")]
    FailedToSetRegistry(#[from] StoreError),

    #[error("Failed to get registry from IPFS")]
    FailedToRetrieve(String),

    #[error("Unsupported version")]
    UnsupportedVersion,

    #[error("Failed to parse into registry")]
    FailedToParse,

    #[error("Registry is invalid: {0}")]
    InvalidRegistry(String),

    #[error("Invalid ipfs hash")]
    InvalidHash,
}

#[derive(Clone, Debug, Error, PartialEq)]
#[error("Signal {signal_id} doesnt exist in the registry")]
pub enum SetActiveSignalError {
    #[error("Signal \"{0}\" doesnt exist in the registry")]
    MissingSignal(String),

    #[error("Failed to set active signal ids")]
    FailedToSetActiveSignalIds(#[from] StoreError),
}
