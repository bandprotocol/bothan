#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid price: {0}")]
    InvalidPrice(f64),

    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(#[from] chrono::ParseError),
}
