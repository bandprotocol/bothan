use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelResponse {
    Ticker(PushData<Vec<TickerData>>),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PushData<T> {
    pub arg: ChannelArgument,
    pub data: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChannelArgument {
    pub channel: String,
    pub inst_id: String,
}

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
