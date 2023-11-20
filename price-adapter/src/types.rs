use core::fmt;

#[derive(Clone, Debug)]
pub struct PriceInfo {
    pub symbol: String,
    pub price: f64,
    pub timestamp: u64,
}

impl fmt::Display for PriceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PriceInfo {{ symbol: {}, price: {}, timestamp: {} }}",
            self.symbol, self.price, self.timestamp
        )
    }
}
