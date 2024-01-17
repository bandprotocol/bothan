use thiserror::Error;

/// Custom error type for the application.
#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("price-adapter-raw error: {0}")]
    PriceAdapterRawError(#[from] price_adapter_raw::error::Error),

    #[error("file error: {0}")]
    FileError(#[from] std::io::Error),

    #[error("serde-json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("unsupported symbol")]
    UnsupportedSymbol,

    #[error("unsupported source")]
    UnsupportedSource,

    #[error("mapping error")]
    MappingError,

    #[error("service already started")]
    AlreadyStarted,

    #[error("service not connected")]
    NotConnected,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("system-time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),
}
