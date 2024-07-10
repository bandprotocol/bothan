#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid price: {0}")]
    InvalidPrice(f64),

    /// The provided timestamp is invalid.
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(#[from] chrono::ParseError),
}
