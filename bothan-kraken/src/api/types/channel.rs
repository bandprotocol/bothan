use serde::{Deserialize, Serialize};

pub mod status;
pub mod ticker;

/// Represents the response from various channels.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "channel", content = "data", rename_all = "snake_case")]
pub enum ChannelResponse {
    /// Response for ticker data.
    Ticker(Vec<ticker::TickerResponse>),
    /// Response for status data.
    Status(Vec<status::Status>),
    /// Heartbeat response.
    Heartbeat,
}
