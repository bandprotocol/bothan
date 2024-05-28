use serde::{Deserialize, Serialize};

use crate::api::types::channel::ChannelResponse;
use crate::api::types::message::WebSocketMessageResponse;

pub mod channel;
pub mod message;

pub const DEFAULT_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum OKXResponse {
    WebSocketMessageResponse(WebSocketMessageResponse),
    ChannelResponse(ChannelResponse),
}
