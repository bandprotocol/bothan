#[derive(Clone, Debug, PartialEq, thiserror::Error)]
/// Errors that can occur while building the `CoinGeckoRestAPI`.
pub enum BuilderError {
    /// An invalid header value was provided.
    #[error("invalid header value: {0}")]
    InvalidHeaderValue(String),

    /// The URL provided is invalid.
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("reqwest error: {0}")]
    Reqwest(String),
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
