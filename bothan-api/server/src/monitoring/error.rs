#[derive(Debug, thiserror::Error)]
pub enum PostError {
    #[error("Failed to serialize data: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to serialize data: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Failed to serialize data: {0}")]
    Serialize(#[from] serde_json::Error),
}
