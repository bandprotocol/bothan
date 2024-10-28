#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to send request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Value is not a valid header: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Failed to serialize data: {0}")]
    Serialize(#[from] serde_json::Error),
}
