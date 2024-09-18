use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("failed to connect to endpoint {0}")]
    ConnectionFailure(#[from] tungstenite::Error),

    #[error("received unsuccessful WebSocket response: {0}")]
    UnsuccessfulWebSocketResponse(tungstenite::http::StatusCode),
}

#[derive(Debug, thiserror::Error)]
pub enum MessageError {
    #[error("failed to parse message")]
    Parse(#[from] serde_json::Error),

    #[error("channel closed")]
    ChannelClosed,

    #[error("unsupported message")]
    UnsupportedMessage,
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error(transparent)]
    Tungstenite(#[from] tungstenite::Error),

    /// Indicates a failure to parse a message.
    #[error("failed to parse")]
    Parse(#[from] serde_json::Error),
}
