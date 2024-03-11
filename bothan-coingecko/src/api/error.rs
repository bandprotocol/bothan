#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(String),

    #[error("invalid header value")]
    InvalidHeaderValue(String),

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("core api error: {0}")]
    CoreApi(#[from] bothan_core::api::error::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(e: reqwest::header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(e.to_string())
    }
}
