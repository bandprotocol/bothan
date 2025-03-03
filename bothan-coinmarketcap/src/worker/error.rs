use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("ids contains non integer value: {0}")]
    InvalidId(String),

    #[error("failed to fetch tickers: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("missing value")]
    MissingValue,

    #[error("value contains nan")]
    InvalidValue,
}
