// TODO: Add more errors apart from catch all
/// Represents various errors that can occur in the service.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An unknown error occurred.
    #[error("unknown error: {0}")]
    Unknown(String),

    /// A result is still pending.
    #[error("pending result")]
    Pending,

    /// The provided symbol is invalid.
    #[error("invalid symbol")]
    InvalidSymbol,

    /// The provided timestamp is invalid.
    #[error("invalid timestamp")]
    InvalidTimestamp,

    /// An API error occurred.
    #[error("api error: {0}")]
    Api(#[from] crate::api::error::RestAPIError),
}
