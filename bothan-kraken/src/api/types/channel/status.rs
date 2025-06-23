//! Types for Kraken WebSocket API status information.
//!
//! This module provides types used for deserializing status responses from the Kraken WebSocket API.
//! The status messages contain metadata about the API version, connection details, and system information.
//!
//! # Key Types
//!
//! - [`Status`] – Structure representing status data from Kraken WebSocket responses.

use serde::{Deserialize, Serialize};

/// Represents the status information from the Kraken WebSocket API.
///
/// `Status` contains metadata about the current connection and API details provided by Kraken.
///
/// # Examples
///
/// ```rust
/// use bothan_kraken::api::types::channel::status::Status;
/// use serde_json::json;
///
/// let status_json = json!({
///     "api_version": "v2",
///     "connection_id": 13834774380200032777,
///     "system": "online",
///     "version": "2.0.0"
/// });
///
/// let status: Status = serde_json::from_value(status_json).unwrap();
/// assert_eq!(status.api_version, "v2");
/// assert_eq!(status.system, "online");
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    /// The version of the websockets API.
    pub api_version: String,

    /// A unique connection identifier.
    pub connection_id: usize,

    /// The status of the trading engine.
    /// - `online`: Markets are operating normally - all order types may be submitted and order matching can occur.
    /// - `maintenance`: Markets are offline for scheduled maintenance - new orders cannot be placed and existing orders cannot be cancelled.
    /// - `cancel_only`: Orders can be cancelled but new orders cannot be placed. No order matching will occur.
    /// - `post_only`: Only limit orders using the `post_only` option can be submitted. Orders can be cancelled. No order matching will occur.
    pub system: String,

    /// The version of the websockets service.
    pub version: String,
}
