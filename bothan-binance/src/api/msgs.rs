use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub result: Option<String>,
    pub id: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i16,
    pub msg: String,
    pub id: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum Data {
    #[serde(rename = "24hrMiniTicker")]
    MiniTicker(MiniTickerInfo),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MiniTickerInfo {
    #[serde(rename = "E")]
    pub event_time: i64,

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamResponse {
    pub stream: String,
    pub data: Data,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BinanceResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
    Stream(StreamResponse),
    Ping,
}
