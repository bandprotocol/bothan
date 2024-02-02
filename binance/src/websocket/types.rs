use serde::Deserialize;

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
