use serde::{Deserialize, Serialize};

/// Represents the arguments for a price request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// The name of the channel.
    pub channel: String,
    /// The type of instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// The instrument family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// The instrument ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
}

/// Represents ticker data received from the channel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    Spot,
}
