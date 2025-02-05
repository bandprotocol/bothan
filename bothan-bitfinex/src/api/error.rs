use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("failed to build with error: {0}")]
    FailedToBuild(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("failed to fetch tickers: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("value contains nan")]
    InvalidValue,
}
