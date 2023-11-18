use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("raw error: {0}")]
    RawError(#[from] price_adapter_raw::error::Error),

    #[error("file error: {0}")]
    FileError(#[from] std::io::Error),
}
