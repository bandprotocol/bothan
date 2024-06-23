// TODO: Add more errors apart from catch all
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Represents an unknown error with a specific message.
    #[error("unknown error: {0}")]
    Unknown(String),

    /// Represents a pending result error.
    #[error("pending result")]
    Pending,

    /// Represents an invalid symbol error.
    #[error("invalid symbol")]
    InvalidSymbol,

    /// Represents an error from the Tungstenite WebSocket library.
    #[error("tungstenite error")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    /// Represents an error from the API.
    #[error("api error: {0}")]
    API(#[from] crate::api::error::Error),
}
