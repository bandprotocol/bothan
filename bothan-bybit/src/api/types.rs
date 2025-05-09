use serde::{Deserialize, Serialize};

/// The default URL for the Bybit WebSocket API.
pub const DEFAULT_URL: &str = "wss://stream.bybit.com/v5/public/spot";
pub const MAX_ARGS: usize = 10;

/// Represents the different types of responses from the Bybit API.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Response {
    /// Represents a public message response with an operation status.
    PublicMessage(PublicMessageResponse),
    /// Represents a public ticker response with market data.
    PublicTicker(PublicTickerResponse),
    /// A ping.
    Ping,
}

/// Struct representing a public message response with an operation status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PublicMessageResponse {
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub req_id: Option<String>,
    pub op: String,
}

/// Struct representing a public ticker response with market data.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PublicTickerResponse {
    pub topic: String,
    pub ts: i64,
    #[serde(rename = "type")]
    pub ticker_type: String,
    pub cs: i64,
    pub data: Ticker,
}

/// Represents the market data structure for the PublicTickerResponse.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub last_price: String,
    pub high_price24h: String,
    pub low_price24h: String,
    pub prev_price24h: String,
    pub volume24h: String,
    pub turnover24h: String,
    pub price24h_pcnt: String,
    pub usd_index_price: String,
}
