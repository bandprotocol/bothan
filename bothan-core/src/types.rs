use derive_more::Display;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Display)]
#[display("id: {}, price: {}, timestamp: {}", id, price, timestamp)]
pub struct PriceData {
    pub id: String,
    pub price: String,
    pub timestamp: u64,
}

impl PriceData {
    pub fn new(id: String, price: String, timestamp: u64) -> Self {
        Self {
            id,
            price,
            timestamp,
        }
    }
}
