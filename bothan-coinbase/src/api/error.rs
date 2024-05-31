use tokio_tungstenite::tungstenite::{self, http::StatusCode};

/// Represents the various errors that can occur.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Connection failure with a response code.
    #[error("failed to connect with response code {0}")]
    ConnectionFailure(StatusCode),

    /// Error parsing JSON.
    #[error("failed to parse")]
    Parse(#[from] serde_json::Error),

    /// Error from the Tungstenite library.
    #[error("tungstenite error")]
    Tungstenite(#[from] tungstenite::Error),

    /// Channel closed unexpectedly.
    #[error("channel closed")]
    ChannelClosed,

    /// Received an unsupported message.
    #[error("unsupported message")]
    UnsupportedMessage,
}
