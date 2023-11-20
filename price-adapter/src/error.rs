use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("price-adapter-raw error: {0}")]
    PriceAdapterRawError(#[from] price_adapter_raw::error::Error),

    #[error("file error: {0}")]
    FileError(#[from] std::io::Error),
}
