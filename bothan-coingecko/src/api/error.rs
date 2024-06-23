#[derive(Clone, Debug, PartialEq, thiserror::Error)]
/// Errors that can occur while building the `CoinGeckoRestAPI`.
pub enum BuilderError {
    /// An error occurred with the `reqwest` client.
    #[error("reqwest error: {0}")]
    Reqwest(String),

    /// An invalid header value was provided.
    #[error("invalid header value")]
    InvalidHeaderValue(String),

    /// The URL provided is invalid.
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
/// Errors that can occur while interacting with the REST API.
pub enum RestAPIError {
    /// An HTTP error occurred.
    #[error("http error: {0}")]
    Http(reqwest::StatusCode),

    /// An invalid ID was provided.
    #[error("invalid id")]
    InvalidID,

    /// Failed to parse the response.
    #[error("failed to parse")]
    Parse,

    /// An error occurred with the `reqwest` client.
    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<reqwest::Error> for RestAPIError {
    fn from(e: reqwest::Error) -> Self {
        RestAPIError::Reqwest(e.to_string())
    }
}
