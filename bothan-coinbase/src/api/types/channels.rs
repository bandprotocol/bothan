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
