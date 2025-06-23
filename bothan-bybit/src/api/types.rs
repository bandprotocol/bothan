//! Types for the Bybit WebSocket API.
//!
//! This module provides types for deserializing events and responses from the Bybit WebSocket API,
//! including public messages, ticker updates, and other market data.

use serde::{Deserialize, Serialize};

/// The default URL for the Bybit WebSocket API.
pub const DEFAULT_URL: &str = "wss://stream.bybit.com/v5/public/spot";
/// The maximum number of arguments allowed in a single WebSocket request.
pub const MAX_ARGS: usize = 10;

/// Represents the different types of responses from the Bybit API.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Response {
    /// Represents a public message response with an operation status.
    PublicMessage(PublicMessageResponse),
    /// Represents a public ticker response with market data.
    PublicTicker(PublicTickerResponse),
    /// Represents a ping message from the WebSocket API.
    Ping,
}

/// Represents a public message response with an operation status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PublicMessageResponse {
    /// Whether the operation was successful.
    pub success: bool,
    /// The return message from the server.
    pub ret_msg: String,
    /// The connection ID.
    pub conn_id: String,
    /// The request ID, if present.
    pub req_id: Option<String>,
    /// The operation type.
    pub op: String,
}

/// Represents a public ticker response with market data.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PublicTickerResponse {
    /// The topic of the ticker.
    pub topic: String,
    /// The timestamp of the ticker event (in milliseconds).
    pub ts: i64,
    /// The type of the ticker event.
    #[serde(rename = "type")]
    pub ticker_type: String,
    /// The cross sequence value, which is a unique identifier for the latest update in the stream.
    pub cs: i64,
    /// The ticker data.
    pub data: Ticker,
}

/// Represents the market data structure for the PublicTickerResponse.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// The trading symbol (e.g., SOLUSDT_SOL/USDT).
    pub symbol: String,
    /// The last traded price.
    pub last_price: String,
    /// The highest price in the last 24 hours.
    pub high_price24h: String,
    /// The lowest price in the last 24 hours.
    pub low_price24h: String,
    /// The previous price 24 hours ago.
    pub prev_price24h: String,
    /// The trading volume in the last 24 hours.
    pub volume24h: String,
    /// The trading turnover in the last 24 hours.
    pub turnover24h: String,
    /// The price change percentage in the last 24 hours.
    pub price24h_pcnt: String,
    /// The USD index price.
    pub usd_index_price: String,
}
