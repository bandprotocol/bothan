pub use channel::{ChannelArgument, PushData};
use serde::{Deserialize, Serialize};

use crate::api::types::ticker::Ticker;

pub mod channel;
pub mod subscription;
pub mod ticker;

pub const DEFAULT_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

/// Represents the different types of responses from the OKX WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Response {
    /// A response from a WebSocket message.
    TickerSubscription(subscription::Response<ticker::Request>),
    /// A response containing data from a subscribed channel.
    TickersChannel(PushData<Vec<Ticker>>),
    Ping,
}
