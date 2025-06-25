//! Ticker type for the Coinbase WebSocket API
//!
//! This module defines the structure of a ticker message received from the Coinbase WebSocket API.
//! It includes fields for price, volume, and other relevant trading data.
use serde::{Deserialize, Serialize};

/// Ticker type for the Coinbase WebSocket API.
///
/// This module defines the structure of a ticker message received from the Coinbase WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticker {
    /// The sequence number of the ticker.
    pub sequence: usize,
    /// The product ID.
    pub product_id: String,
    /// The price of the product.
    pub price: String,
    /// The opening price of the product in the last 24 hours.
    pub open_24h: String,
    /// The volume of the product traded in the last 24 hours.
    pub volume_24h: String,
    /// The lowest price of the product in the last 24 hours.
    pub low_24h: String,
    /// The highest price of the product in the last 24 hours.
    pub high_24h: String,
    /// The volume of the product traded in the last 30 days.
    pub volume_30d: String,
    /// The best bid price.
    pub best_bid: String,
    /// The size of the best bid.
    pub best_bid_size: String,
    /// The best ask price.
    pub best_ask: String,
    /// The size of the best ask.
    pub best_ask_size: String,
    /// The side of the last trade (buy/sell).
    pub side: String,
    /// The timestamp of the ticker.
    pub time: String,
    /// The trade ID of the last trade.
    pub trade_id: usize,
    /// The size of the last trade.
    pub last_size: String,
}
