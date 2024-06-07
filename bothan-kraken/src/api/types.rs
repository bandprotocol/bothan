use serde::{Deserialize, Serialize};

use crate::api::types::channel::ChannelResponse;
use crate::api::types::message::PublicMessageResponse;

pub mod channel;
pub mod message;

/// The default URL for the Kraken WebSocket API.
pub const DEFAULT_URL: &str = "wss://ws.kraken.com/v2";

/// Represents the different types of responses from the Kraken API.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum KrakenResponse {
    /// A response for public messages.
    PublicMessage(PublicMessageResponse),
    /// A response from a channel subscription.
    Channel(ChannelResponse),
    /// A response from a ping.
    Pong,
}
