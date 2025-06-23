//! Types for Kraken WebSocket channel responses.
//!
//! This module provides types for deserializing responses from various Kraken WebSocket API channels.
//! Each variant corresponds to a specific Kraken WebSocket channel response format, allowing easy handling
//! of real-time market data, status updates, and heartbeats.
//!
//! # Key Types
//!
//! - [`ChannelResponse`] – Enum for Kraken channel-specific WebSocket responses.

use serde::{Deserialize, Serialize};

pub mod status;
pub mod ticker;

/// Represents responses from Kraken WebSocket channels.
///
/// Each variant corresponds to a specific type of channel data from the Kraken API.
///
/// # Examples
///
/// ```rust
/// use bothan_kraken::api::types::channel::{ChannelResponse, ticker::TickerResponse};
/// use serde_json::json;
///
/// let response_json = json!({
///     "channel": "ticker",
///     "data": [
///         {
///             "symbol": "BTC/USD",
///             "bid": 30000.0,
///             "ask": 30001.0,
///             "last": 30000.5,
///             "volume": 12.5,
///             "vwap": 29950.0,
///             "low": 29500.0,
///             "high": 30500.0,
///             "change": 500.0,
///             "change_pct": 0.0169
///         }
///     ]
/// });
///
/// let response: ChannelResponse = serde_json::from_value(response_json).unwrap();
///
/// if let ChannelResponse::Ticker(data) = response {
///     assert_eq!(data[0].symbol, "BTC/USD");
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "channel", content = "data", rename_all = "snake_case")]
pub enum ChannelResponse {
    /// Ticker data updates from subscribed instruments.
    Ticker(Vec<ticker::TickerResponse>),

    /// Status data updates indicating connection and service status.
    Status(Vec<status::Status>),

    /// Heartbeat messages indicating an active connection.
    Heartbeat,
}
