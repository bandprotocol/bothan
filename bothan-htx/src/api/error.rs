#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum BuilderError {
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<reqwest::Error> for BuilderError {
    fn from(e: reqwest::Error) -> Self {
        BuilderError::Reqwest(e.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum RestApiError {
    #[error("http error: {0}")]
    Http(reqwest::StatusCode),

    // #[error("serde e
    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<reqwest::Error> for RestApiError {
    fn from(e: reqwest::Error) -> Self {
        RestApiError::Reqwest(e.to_string())
    }
}
