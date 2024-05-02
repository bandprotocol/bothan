use derive_more::Display;

#[derive(Clone, PartialEq, Debug, Display)]
#[display("PriceData{{id: {id}, price: {price}, timestamp: {timestamp}}}")]
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
