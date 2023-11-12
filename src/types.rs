use core::fmt;

#[derive(Clone, Debug)]
pub struct PriceInfo {
    pub base: String,
    pub quote: String,
    pub price: f64,
    pub timestamp: u64,
}

impl fmt::Display for PriceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PriceInfo {{ base: {}, quote: {}, price: {}, timestamp: {} }}",
            self.base, self.quote, self.price, self.timestamp
        )
    }
}
