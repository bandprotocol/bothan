pub use channel::{ChannelArgument, ChannelResponse, PushData, TickerData};
pub use message::WebSocketMessageResponse;
use serde::{Deserialize, Serialize};

pub mod channel;
pub mod message;

pub const DEFAULT_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

/// Represents the different types of responses from the OKX WebSocket API.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum OkxResponse {
    /// A response from a WebSocket message.
    WebSocketMessageResponse(WebSocketMessageResponse),
    /// A response containing data from a subscribed channel.
    ChannelResponse(ChannelResponse),
}
