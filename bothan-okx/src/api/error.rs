use std::io;

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
pub enum PollingError {
    #[error(transparent)]
    Error(#[from] Error),

    #[error(transparent)]
    InvalidPrice(#[from] rust_decimal::Error),

    #[error(transparent)]
    InvalidTimestamp(#[from] std::num::ParseIntError),
}
