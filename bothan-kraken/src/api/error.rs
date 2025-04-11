use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read message")]
    IO(#[from] io::Error),

    #[error("failed to parse message")]
    ParseError(#[from] serde_json::Error),

    #[error("unsupported message")]
    UnsupportedWebsocketMessageType,
}

#[derive(Debug, thiserror::Error)]
pub enum PollingError {
    #[error(transparent)]
    Error(#[from] Error),

    #[error("received invalid channel id")]
    InvalidChannelId,

    #[error("received NaN")]
    InvalidPrice,
}
