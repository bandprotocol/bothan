#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("temporary catch all")]
    CatchAll,

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("invalid header value")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("http error: {0}")]
    Http(reqwest::StatusCode),
}
