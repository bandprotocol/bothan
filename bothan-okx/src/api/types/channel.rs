use serde::{Deserialize, Serialize};

/// Represents a response from a channel subscription.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelResponse {
    /// Ticker data response.
    Ticker(PushData<Vec<TickerData>>),
}

/// Represents push data from a channel.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PushData<T> {
    /// The argument for the channel.
    pub arg: ChannelArgument,
    /// The data received from the channel.
    pub data: T,
}

/// Represents the argument for a channel.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelArgument {
    /// The name of the channel.
    pub channel: String,
    /// The instrument ID.
    pub inst_id: String,
}

/// Represents ticker data received from the channel.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TickerData {
    pub inst_type: String,
    pub inst_id: String,
    pub last: String,
    pub last_sz: String,
    pub ask_px: String,
    pub ask_sz: String,
    pub bid_px: String,
    pub bid_sz: String,
    pub open_24h: String,
    pub high_24h: String,
    pub low_24h: String,
    pub vol_ccy_24h: String,
    pub vol_24h: String,
    pub sod_utc0: String,
    pub sod_utc8: String,
    pub ts: String,
}
