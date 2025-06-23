//! Types for OKX WebSocket channel data.
//!
//! This module provides types for handling channel-based data from the OKX WebSocket API,
//! including push data structures and channel arguments. These types are used for
//! processing real-time market data updates from subscribed channels.
//!
//! # Key Types
//!
//! - [`PushData<T>`] - Generic container for channel push data
//! - [`ChannelArgument`] - Channel identification and metadata

use serde::{Deserialize, Serialize};

/// Represents push data from a channel.
///
/// This generic struct contains data received from a subscribed OKX WebSocket channel.
/// It includes both the channel metadata (`arg`) and the actual data payload (`data`).
///
/// # Examples
///
/// ```rust
/// use bothan_okx::api::types::channel::{PushData, ChannelArgument};
///
/// let arg = ChannelArgument {
///     channel: "tickers".to_string(),
///     inst_id: "BTC-USDT".to_string(),
/// };
///
/// let push_data = PushData {
///     arg,
///     data: vec!["some", "ticker", "data"],
/// };
///
/// assert_eq!(push_data.arg.channel, "tickers");
/// assert_eq!(push_data.arg.inst_id, "BTC-USDT");
/// ```
///
/// # OKX API Response Example
///
/// ```json
/// {
///   "arg": {
///     "channel": "tickers",
///     "instId": "BTC-USDT"
///   },
///   "data": [
///     {
///       "instType": "SPOT",
///       "instId": "BTC-USDT",
///       "last": "50000",
///       "lastSz": "0.1",
///       "askPx": "50001",
///       "askSz": "1.5",
///       "bidPx": "49999",
///       "bidSz": "2.0",
///       "open24h": "49000",
///       "high24h": "51000",
///       "low24h": "48000",
///       "volCcy24h": "1000000",
///       "vol24h": "20",
///       "sodUtc0": "49000",
///       "sodUtc8": "49000",
///       "ts": "1640995200000"
///     }
///   ]
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushData<T> {
    /// The argument for the channel containing metadata.
    pub arg: ChannelArgument,

    /// The data received from the channel.
    pub data: T,
}

/// Represents the argument for a channel.
///
/// This struct contains metadata about a WebSocket channel, including the channel name
/// and the specific instrument being monitored. It's used to identify the source and
/// context of incoming data.
///
/// # Examples
///
/// ```rust
/// use bothan_okx::api::types::channel::ChannelArgument;
///
/// let arg = ChannelArgument {
///     channel: "tickers".to_string(),
///     inst_id: "BTC-USDT".to_string(),
/// };
///
/// assert_eq!(arg.channel, "tickers");
/// assert_eq!(arg.inst_id, "BTC-USDT");
/// ```
///
/// # OKX API Channel Argument Example
///
/// ```json
/// {
///   "channel": "tickers",
///   "instId": "BTC-USDT"
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelArgument {
    /// The name of the channel (e.g., "tickers", "trades", "books").
    pub channel: String,

    /// The instrument ID being monitored (e.g., "BTC-USDT").
    pub inst_id: String,
}
