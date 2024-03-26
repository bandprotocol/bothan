#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum BuilderError {
    #[error("missing api key")]
    MissingAPIKey(),

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("invalid header value")]
    InvalidHeaderValue(String),

    #[error("reqwest error: {0}")]
    Reqwest(String),
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
    #[error("limit must be lower or equal to 5000")]
    LimitTooHigh(),

    #[error("http error: {0}")]
    Http(reqwest::StatusCode),

    #[error("invalid id")]
    InvalidID,

    #[error("failed to parse")]
    Parse,

    #[error("serde error: {0}")]
    Serde(String),

    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<serde_json::Error> for RestApiError {
    fn from(e: serde_json::Error) -> Self {
        RestApiError::Serde(e.to_string())
    }
}

impl From<reqwest::Error> for RestApiError {
    fn from(e: reqwest::Error) -> Self {
        RestApiError::Reqwest(e.to_string())
    }
}
