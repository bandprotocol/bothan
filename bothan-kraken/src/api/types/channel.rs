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
