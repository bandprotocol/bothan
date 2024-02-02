use core::fmt;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PriceData {
    pub id: String,
    pub price: String,
    pub timestamp: u64,
}

impl fmt::Display for PriceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PriceInfo {{ id: {}, price: {}, timestamp: {} }}",
            self.id, self.price, self.timestamp
        )
    }
}

#[derive(Debug, Deserialize)]
pub enum ServiceMessage {
    Relay(String),
    Close(Option<String>),
}
