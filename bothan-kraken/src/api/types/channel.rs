use serde::{Deserialize, Serialize};

pub mod status;
pub mod ticker;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "channel", content = "data", rename_all = "snake_case")]
pub enum ChannelResponse {
    Ticker(Vec<ticker::TickerResponse>),
    Status(Vec<status::Status>),
    Heartbeat,
}
