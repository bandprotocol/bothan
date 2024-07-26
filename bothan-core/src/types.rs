use derive_more::Display;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Display, Serialize, Deserialize)]
#[display("AssetInfo{{id: {id}, price: {price}, timestamp: {timestamp}}}")]
pub struct AssetInfo {
    pub id: String,
    pub price: Decimal,
    pub timestamp: i64,
}

impl AssetInfo {
    pub fn new(id: String, price: Decimal, timestamp: i64) -> Self {
        Self {
            id,
            price,
            timestamp,
        }
    }
}
