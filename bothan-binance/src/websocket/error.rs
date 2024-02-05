use thiserror::Error;
use tokio_tungstenite::tungstenite::{self, http::StatusCode};
use tracing::error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,

    #[error("failed to connect with response code {0}")]
    ConnectionError(StatusCode),

    #[error("failed to parse")]
    ParseError(#[from] serde_json::Error),

    #[error("not connected")]
    NotConnected(),

    #[error("already connected")]
    AlreadyConnected(),

    #[error("tungstenite error")]
    TungsteniteError(#[from] tungstenite::Error),
}
