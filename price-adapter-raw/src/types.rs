use core::fmt;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct PriceInfo {
    pub id: String,
    pub price: f64,
    pub timestamp: u64,
}

impl fmt::Display for PriceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PriceInfo {{ id: {}, price: {}, timestamp: {} }}",
            self.id, self.price, self.timestamp
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct SettingResponse {
    pub data: Value,
}

#[derive(Debug)]
pub enum WebsocketMessage {
    PriceInfo(PriceInfo),
    SettingResponse(SettingResponse),
}
