use crate::registry::validate::ValidationError;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("An error occurred while storing the data: {message}")]
pub struct Error {
    message: String,
}

impl From<ValidationError> for Error {
    fn from(error: ValidationError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<rust_rocksdb::Error> for Error {
    fn from(error: rust_rocksdb::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<bincode::error::EncodeError> for Error {
    fn from(error: bincode::error::EncodeError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<bincode::error::DecodeError> for Error {
    fn from(error: bincode::error::DecodeError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
