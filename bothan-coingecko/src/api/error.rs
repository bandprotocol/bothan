#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum BuildError {
    #[error("invalid header value: {0}")]
    InvalidHeaderValue(String),

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("build failed with error: {0}")]
    BuildFailed(String),
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum SendError {
    #[error("failed to send request with error: {0}")]
    FailedRequest(String),

    #[error("received non-2xx http status: {0}")]
    UnsuccessfulResponse(reqwest::StatusCode),

    #[error("failed to parse with error: {0}")]
    ParseFailed(String),
}
