#[derive(Debug, Clone, PartialEq)]
pub enum PriceState {
    Available(i64),
    Unavailable,
    Unsupported,
}
