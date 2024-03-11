#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(String),

    #[error("http error: {0}")]
    Http(reqwest::StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e.to_string())
    }
}
