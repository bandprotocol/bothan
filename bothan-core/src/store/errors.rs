use bincode::ErrorKind;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("An error occurred while storing the data: {message}")]
pub struct Error {
    message: String,
}

impl From<rust_rocksdb::Error> for Error {
    fn from(error: rust_rocksdb::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl From<Box<ErrorKind>> for Error {
    fn from(error: Box<ErrorKind>) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
