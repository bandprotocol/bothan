#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum BuilderError {
    #[error("reqwest error: {0}")]
    Reqwest(String),

    #[error("invalid header value")]
    InvalidHeaderValue(String),

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),
}

impl From<reqwest::header::InvalidHeaderValue> for BuilderError {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        BuilderError::InvalidHeaderValue(e.to_string())
    }
}

impl From<reqwest::Error> for BuilderError {
    fn from(e: reqwest::Error) -> Self {
        BuilderError::Reqwest(e.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum RestApiError {
    #[error("reqwest error: {0}")]
    Reqwest(String),

    #[error("http error: {0}")]
    Http(reqwest::StatusCode),

    #[error("invalid id")]
    InvalidID,

    #[error("failed to parse")]
    Parse,
}

impl From<reqwest::Error> for RestApiError {
    fn from(e: reqwest::Error) -> Self {
        RestApiError::Reqwest(e.to_string())
    }
}
