use tokio_tungstenite::tungstenite::{self, http::StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect with response code {0}")]
    ConnectionFailure(StatusCode),

    #[error("failed to parse")]
    Parse(#[from] serde_json::Error),

    #[error("not connected")]
    NotConnected(),

    #[error("already connected")]
    AlreadyConnected(),

    #[error("tungstenite error")]
    Tungstenite(#[from] tungstenite::Error),

    #[error("internal channel closed")]
    ChannelClosed,
}