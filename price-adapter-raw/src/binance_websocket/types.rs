use serde::Deserialize;
use serde_json::Value;

use crate::types::PriceInfo;

#[derive(Debug)]
pub enum RequestMethod {
    Subscribe,
    Unsubscribe,
}

impl std::fmt::Display for RequestMethod {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", format!("{:?}", self).to_uppercase())
    }
}

#[derive(Debug, Deserialize)]
pub struct MiniTickerInfo {
    #[serde(rename = "s")]
    pub id: String,

    #[serde(rename = "c")]
    pub current_price: String,

    #[serde(rename = "E")]
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct MiniTickerResponse {
    pub stream: String,
    pub data: MiniTickerInfo,
}

#[derive(Debug, Deserialize)]
pub struct SettingResponse {
    pub result: Value,
    pub id: u64,
}

#[derive(Debug)]
pub enum WebsocketMessage {
    PriceInfo(PriceInfo),
    SettingResponse(SettingResponse),
}
