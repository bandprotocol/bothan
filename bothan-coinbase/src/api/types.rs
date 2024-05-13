use serde::{Deserialize, Serialize};

use crate::api::types::channels::ticker::Ticker;
use crate::api::types::request::Subscriptions;

pub mod channels;
pub mod request;

pub const DEFAULT_URL: &str = "wss://ws-feed.exchange.coinbase.com";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum CoinbaseResponse {
    Ticker(Box<Ticker>),
    Subscriptions(Box<Subscriptions>),
}
