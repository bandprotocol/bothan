use serde::{Deserialize, Serialize};

use crate::api::types::channel::ChannelResponse;
use crate::api::types::message::PublicMessageResponse;

pub mod channel;
pub mod message;

pub const DEFAULT_URL: &str = "wss://ws.kraken.com/v2";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum KrakenResponse {
    PublicMessage(PublicMessageResponse),
    Channel(ChannelResponse),
    Pong,
}
