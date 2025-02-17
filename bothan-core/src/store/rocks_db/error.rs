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

// #[derive(Debug)]
// struct Error {
//     message: String,
// }
//
// // Implement std::fmt::Display for your custom Error type
// impl Display for Error {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "{}", self.message)
//     }
// }
//
// // Implement std::error::Error for your custom Error type
// impl StdError for Error {}
//

// Implement a From conversion for any type that implements std::error::Error
// impl<E: StdError> From<E> for Error {
//     fn from(err: E) -> Self {
//         Error {
//             message: err.to_string(),
//         }
//     }
// }

// #[derive(Clone, Debug, thiserror::Error, PartialEq)]
// #[error("An error occurred while storing the data: {message}")]
// pub struct Error {
//     message: String,
// }

//
// impl From<ValidationError> for Error {
//     fn from(error: ValidationError) -> Self {
//         Self {
//             message: error.to_string(),
//         }
//     }
// }
//
// impl From<rust_rocksdb::Error> for Error {
//     fn from(error: rust_rocksdb::Error) -> Self {
//         Self {
//             message: error.to_string(),
//         }
//     }
// }
//
// impl From<bincode::error::EncodeError> for Error {
//     fn from(error: bincode::error::EncodeError) -> Self {
//         Self {
//             message: error.to_string(),
//         }
//     }
// }
//
// impl From<bincode::error::DecodeError> for Error {
//     fn from(error: bincode::error::DecodeError) -> Self {
//         Self {
//             message: error.to_string(),
//         }
//     }
// }
