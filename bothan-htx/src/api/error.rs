#[derive(Clone, Debug, PartialEq, thiserror::Error)]
/// Errors that can occur while building the `HtxRestAPI`.
pub enum BuildError {
    /// The URL provided is invalid.
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    /// An error occurred with the `reqwest` client.
    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<reqwest::Error> for BuildError {
    fn from(e: reqwest::Error) -> Self {
        BuildError::Reqwest(e.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
/// Errors that can occur while interacting with the REST API.
pub enum RestAPIError {
    /// An HTTP error occurred.
    #[error("http error: {0}")]
    Http(reqwest::StatusCode),

    /// An error occurred with the `reqwest` client.
    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<reqwest::Error> for RestAPIError {
    fn from(e: reqwest::Error) -> Self {
        RestAPIError::Reqwest(e.to_string())
    }
}
