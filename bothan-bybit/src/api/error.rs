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
pub enum RestAPIError {
    #[error("http error: {0}")]
    Http(reqwest::StatusCode),

    #[error("reqwest error: {0}")]
    Reqwest(String),

    #[error("category currently unsupported")]
    UnsupportedCategory,
}

impl From<reqwest::Error> for RestAPIError {
    fn from(e: reqwest::Error) -> Self {
        RestAPIError::Reqwest(e.to_string())
    }
}
