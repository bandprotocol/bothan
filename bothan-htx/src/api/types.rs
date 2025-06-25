//! Types for HTX WebSocket API interaction.
//!
//! This module provides types for deserializing events and responses from the HTX WebSocket API,
//! including subscription responses, data updates, ping/pong messages, and error responses.
//! The module supports gzip-compressed binary messages from the HTX WebSocket stream.

use serde::{Deserialize, Serialize};

/// The default URL for the HTX WebSocket API.
pub const DEFAULT_URL: &str = "wss://api.huobi.pro/ws";

/// Represents the different types of responses that can be received from the HTX WebSocket API.
///
/// The `Response` enum can represent various types of messages from the HTX WebSocket API,
/// including subscription confirmations, data updates, ping messages, and error responses.
/// Each variant corresponds to a specific type of message, allowing for flexible handling
/// of various response types.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Response {
    /// Represents a successful subscription confirmation.
    Subscribed(Subscribed),
    /// Represents a successful unsubscription confirmation.
    Unsubscribed(Unsubscribed),
    /// Represents a market data update with ticker information.
    DataUpdate(Data),
    /// Represents a ping message from the WebSocket API.
    Ping(Ping),
    /// Represents an error response from the WebSocket API.
    Error(Error),
}

/// Represents a successful subscription response from the HTX WebSocket API.
///
/// This struct contains information about a successful subscription to a market data stream,
/// including the subscription ID, status, subscribed channel, and timestamp.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Subscribed {
    /// Optional subscription identifier.
    pub id: Option<String>,
    /// Status of the subscription request (typically "ok").
    pub status: String,
    /// The channel that was successfully subscribed to.
    pub subbed: String,
    /// Unix timestamp in milliseconds when the subscription was processed.
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

/// Represents a successful unsubscription response from the HTX WebSocket API.
///
/// This struct contains information about a successful unsubscription from a market data stream,
/// including the unsubscription ID, status, unsubscribed channel, and timestamp.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Unsubscribed {
    /// Optional unsubscription identifier.
    pub id: Option<String>,
    /// Status of the unsubscription request (typically "ok").
    pub status: String,
    /// The channel that was successfully unsubscribed from.
    pub unsubbed: String,
    /// Unix timestamp in milliseconds when the unsubscription was processed.
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

/// Represents a market data update from the HTX WebSocket API.
///
/// This struct contains market data information including the channel name,
/// timestamp, and detailed ticker information.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    /// The channel name for the market data (e.g., "market.btcusdt.ticker").
    pub ch: String,
    /// Unix timestamp in milliseconds when the data was generated.
    #[serde(rename = "ts")]
    pub timestamp: i64,
    /// Detailed ticker information containing market data.
    pub tick: Tick,
}

/// Represents detailed ticker information from the HTX WebSocket API.
///
/// This struct contains comprehensive market data including price information,
/// volume data, and bid/ask details for a specific trading pair.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tick {
    /// Opening price of last 24 hours (rotating 24h).
    pub open: f64,
    /// Highest price of last 24 hours (rotating 24h).
    pub high: f64,
    /// Lowest price of last 24 hours (rotating 24h).
    pub low: f64,
    /// Closing price of last 24 hours (rotating 24h).
    pub close: f64,
    /// Accumulated trading volume of last 24 hours (rotating 24h), in base currency.
    pub amount: f64,
    /// Accumulated trading value of last 24 hours (rotating 24h), in quote currency.
    pub vol: f64,
    /// The number of completed trades (rotating 24h).
    pub count: u64,
    /// Current best bid price.
    pub bid: f64,
    /// Current best bid size.
    pub bid_size: f64,
    /// Current best ask price.
    pub ask: f64,
    /// Current best ask size.
    pub ask_size: f64,
    /// Last traded price.
    pub last_price: f64,
    /// Size of the last trade.
    pub last_size: f64,
}

/// Represents a ping message from the HTX WebSocket API.
///
/// This struct contains a ping message used for connection keep-alive.
/// The ping value is typically a timestamp that should be echoed back in a pong response.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ping {
    /// The ping value, typically a timestamp.
    pub ping: u64,
}

/// Represents a pong message to send to the HTX WebSocket API.
///
/// This struct contains a pong message used to respond to ping messages
/// for connection keep-alive. The pong value should match the ping value received.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pong {
    /// The pong value, typically echoing the ping value received.
    pub pong: u64,
}

/// Represents an error response from the HTX WebSocket API.
///
/// This struct contains error information when the API returns an error response,
/// including error codes, messages, and timestamps.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Error {
    /// Status of the error response.
    pub status: String,
    /// Unix timestamp in milliseconds when the error occurred.
    pub ts: u64,
    /// Error code indicating the type of error.
    pub err_code: String,
    /// Human-readable error message.
    pub err_msg: String,
}
