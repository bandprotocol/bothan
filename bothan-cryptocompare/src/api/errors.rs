use thiserror::Error;
use tokio_tungstenite::tungstenite;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    #[error("failed to connect to endpoint {0}")]
    ConnectionFailure(#[from] tungstenite::Error),

    #[error("received unsuccessful WebSocket response: {0}")]
    UnsuccessfulWebSocketResponse(tungstenite::http::StatusCode),
}

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("failed to parse message")]
    Parse(#[from] serde_json::Error),

    #[error("channel closed")]
    ChannelClosed,

    #[error("unsupported message")]
    UnsupportedMessage,
}
