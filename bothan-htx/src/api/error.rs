use std::io;

use tokio_tungstenite::tungstenite;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read message")]
    Io(#[from] io::Error),

    #[error("failed to parse message")]
    ParseError(#[from] serde_json::Error),

    #[error("unsupported message")]
    UnsupportedWebsocketMessageType,
}

#[derive(Debug, thiserror::Error)]
pub enum ListeningError {
    #[error(transparent)]
    Error(#[from] Error),

    #[error("received invalid channel id")]
    InvalidChannelId,

    #[error("received NaN")]
    InvalidPrice,

    #[error("failed to pong")]
    PongFailed(#[from] tungstenite::Error),
}
