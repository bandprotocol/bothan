use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("missing api key")]
    MissingAPIKey,

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("invalid header value")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("reqwest error: {0}")]
    FailedToBuild(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("limit must be lower or equal to 5000")]
    LimitTooHigh,

    #[error("failed request: {0}")]
    FailedRequest(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("ids contains non integer value")]
    InvalidId,

    #[error("failed to fetch tickers: {0}")]
    RequestError(#[from] reqwest::Error),
}
