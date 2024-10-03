#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum BuildError {
    #[error("missing api key")]
    MissingAPIKey(),

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("invalid header value")]
    InvalidHeaderValue(String),

    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<reqwest::header::InvalidHeaderValue> for BuildError {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        BuildError::InvalidHeaderValue(e.to_string())
    }
}

impl From<reqwest::Error> for BuildError {
    fn from(e: reqwest::Error) -> Self {
        BuildError::Reqwest(e.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum RestAPIError {
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

impl From<serde_json::Error> for RestAPIError {
    fn from(e: serde_json::Error) -> Self {
        RestAPIError::Serde(e.to_string())
    }
}

impl From<reqwest::Error> for RestAPIError {
    fn from(e: reqwest::Error) -> Self {
        RestAPIError::Reqwest(e.to_string())
    }
}
