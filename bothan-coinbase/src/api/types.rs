use serde::{Deserialize, Serialize};

use crate::api::types::channels::ticker::Ticker;
use crate::api::types::request::{Error, Subscriptions};

pub mod channels;
pub mod request;

/// The default URL for the Coinbase WebSocket feed.
pub const DEFAULT_URL: &str = "wss://ws-feed.exchange.coinbase.com";

/// Represents the possible responses from the Coinbase feed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Response {
    /// A ticker update.
    Ticker(Box<Ticker>), // Boxed due to large size
    /// A subscription update.
    Subscriptions(Subscriptions),
    Ping,
    Error(Error),
}
