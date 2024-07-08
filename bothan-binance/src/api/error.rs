use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("failed to connect to endpoint {0}")]
    ConnectionFailure(tungstenite::Error),

    #[error("received unsuccessful HTTP response: {0}")]
    UnsuccessfulHttpResponse(tungstenite::http::StatusCode),
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
#[error(transparent)]
pub struct SendError {
    #[from]
    source: tungstenite::Error,
}
