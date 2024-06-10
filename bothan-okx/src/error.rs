/// Enumeration of possible errors that can occur in the service.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Represents an unknown error with a message.
    #[error("unknown error: {0}")]
    Unknown(String),

    /// Indicates a pending result.
    #[error("pending result")]
    Pending,

    /// Indicates an invalid symbol.
    #[error("invalid symbol")]
    InvalidSymbol,

    /// Represents an error from the `tokio_tungstenite` crate.
    #[error("tungstenite error")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    /// Represents an API error with an underlying error from the `api` module.
    #[error("api error: {0}")]
    API(#[from] crate::api::error::Error),
}
