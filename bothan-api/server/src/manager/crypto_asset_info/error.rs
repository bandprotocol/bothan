use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetRegistryError {
    #[error("Failed to get registry from IPFS")]
    FailedToRetrieve,

    #[error("Unsupported version")]
    UnsupportedVersion,

    #[error("Invalid registry")]
    InvalidRegistry,
}
