#[derive(Clone, Debug, PartialEq, thiserror::Error)]
/// Errors that can occur while building the `BybitRestAPI`.
pub enum BuilderError {
    #[error("invalid url")]
    /// The URL provided is invalid.
    InvalidURL(#[from] url::ParseError),

    #[error("reqwest error: {0}")]
    /// An error occurred with the `reqwest` client.
    Reqwest(String),
}

impl From<reqwest::Error> for BuilderError {
    fn from(e: reqwest::Error) -> Self {
        BuilderError::Reqwest(e.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
/// Errors that can occur while interacting with the `BybitRestAPI`.
pub enum RestAPIError {
    #[error("http error: {0}")]
    /// An HTTP error occurred.
    Http(reqwest::StatusCode),

    #[error("reqwest error: {0}")]
    /// An error occurred with the `reqwest` client.
    Reqwest(String),

    #[error("category currently unsupported")]
    /// The specified category is currently unsupported.
    UnsupportedCategory,
}

impl From<reqwest::Error> for RestAPIError {
    fn from(e: reqwest::Error) -> Self {
        RestAPIError::Reqwest(e.to_string())
    }
}
