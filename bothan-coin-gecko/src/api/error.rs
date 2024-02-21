#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(String),

    #[error("invalid header value")]
    InvalidHeaderValue(String),

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("http error: {0}")]
    Http(reqwest::StatusCode),

    #[error("invalid id")]
    InvalidID,

    #[error("failed to parse")]
    Parse,
}
