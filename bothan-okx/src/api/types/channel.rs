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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelArgument {
    /// The name of the channel (e.g., "tickers", "trades", "books").
    pub channel: String,

    /// The instrument ID being monitored (e.g., "BTC-USDT").
    pub inst_id: String,
}
