use serde_json::Error as SerdeError;
use tokio_tungstenite::tungstenite::{self, http::StatusCode};

/// Represents the various errors that can occur.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error indicating a connection failure with a specific status code.
    #[error("failed to connect with response code {0}")]
    ConnectionFailure(StatusCode),

    /// Error indicating a failure to parse a JSON response.
    #[error("failed to parse")]
    Parse(#[from] SerdeError),

    /// Error indicating an issue with the Tungstenite WebSocket library.
    #[error("tungstenite error")]
    Tungstenite(#[from] tungstenite::Error),

    /// Error indicating that the WebSocket channel has closed.
    #[error("channel closed")]
    ChannelClosed,

    /// Error indicating that the message is unsupported.
    #[error("unsupported message")]
    UnsupportedMessage,
}
