use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Ticker {
    pub sequence: usize,
    pub product_id: String,
    pub price: String,
    pub open_24h: String,
    pub volume_24h: String,
    pub low_24h: String,
    pub high_24h: String,
    pub volume_30d: String,
    pub best_bid: String,
    pub best_bid_size: String,
    pub best_ask: String,
    pub best_ask_size: String,
    pub side: String,
    pub time: String,
    pub trade_id: usize,
    pub last_size: String,
}
