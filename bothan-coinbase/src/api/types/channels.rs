//! Channel types for the Coinbase WebSocket API.
//!
//! This module defines the available channels for the Coinbase WebSocket API and their representations.

use serde::{Deserialize, Serialize};

/// Module containing ticker-related types and functions.
pub mod ticker;

/// Represents the different channels available in the API.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    /// A channel for individual ticker updates.
    Ticker,
    /// A channel for batch ticker updates.
    TickerBatch,
}

impl AsRef<str> for Channel {
    fn as_ref(&self) -> &str {
        match self {
            Channel::Ticker => "ticker",
            Channel::TickerBatch => "ticker_batch",
        }
    }
}
