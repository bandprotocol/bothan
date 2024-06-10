use tokio_tungstenite::tungstenite::{self, http::StatusCode};

/// Represents various errors that can occur in the application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Indicates a failure to connect, with the given response status code.
    #[error("failed to connect with response code {0}")]
    ConnectionFailure(StatusCode),

    /// Indicates a failure to parse a message.
    #[error("failed to parse")]
    Parse(#[from] serde_json::Error),

    /// Represents an error from the Tungstenite library.
    #[error("tungstenite error")]
    Tungstenite(#[from] tungstenite::Error),

    /// Indicates that the channel was closed unexpectedly.
    #[error("channel closed")]
    ChannelClosed,

    /// Indicates that an unsupported message was received.
    #[error("unsupported message")]
    UnsupportedMessage,
}
