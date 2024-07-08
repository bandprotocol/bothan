use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SuccessResponse {
    pub result: Option<String>,
    pub id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub code: u16,
    pub msg: String,
    pub id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "e")]
pub enum Data {
    #[serde(rename = "24hrMiniTicker")]
    MiniTicker(MiniTickerInfo),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MiniTickerInfo {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub close_price: String,

    #[serde(rename = "o")]
    pub open_price: String,

    #[serde(rename = "h")]
    pub high_price: String,

    #[serde(rename = "l")]
    pub low_price: String,

    #[serde(rename = "v")]
    pub base_volume: String,

    #[serde(rename = "q")]
    pub quote_volume: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StreamResponse {
    pub stream: String,
    pub data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BinanceResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
    Stream(StreamResponse),
    Ping,
}
