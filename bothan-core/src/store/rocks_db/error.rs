use bincode::error::{DecodeError, EncodeError};
use bothan_lib::registry::ValidationError;

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("No existing registry found")]
    NoExistingRegistry,
    #[error("Failed to decode registry: {0}")]
    FailedToDecodeRegistry(#[from] DecodeError),
    #[error("{0}")]
    RocksDBError(#[from] rust_rocksdb::Error),
    #[error("registry is not valid: {0}")]
    InvalidRegistry(#[from] ValidationError),
}

#[derive(Debug, thiserror::Error)]
pub enum RocksDbError {
    #[error("Failed to encode data: {0}")]
    FailedToeEncode(#[from] EncodeError),

    #[error("Failed to decode data: {0}")]
    FailedToDecode(#[from] DecodeError),

    #[error("{0}")]
    RocksDBError(#[from] rust_rocksdb::Error),
}
