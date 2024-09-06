use serde::{Deserialize, Serialize};

pub use crate::api::types::channels::ticker::Ticker;
use crate::api::types::request::Subscriptions;

pub mod channels;
pub mod request;

/// The default URL for the Coinbase WebSocket feed.
pub const DEFAULT_URL: &str = "wss://ws-feed.exchange.coinbase.com";

/// Represents the possible responses from the Coinbase feed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum CoinbaseResponse {
    /// A ticker update.
    Ticker(Box<Ticker>),
    /// A subscription update.
    Subscriptions(Box<Subscriptions>),
}
