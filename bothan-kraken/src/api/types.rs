//! Types for Kraken WebSocket API interaction.
//!
//! This module provides types for deserializing events and responses from the Kraken WebSocket API,
//! including subscription responses, data updates, and ping messages. The module supports
//! the Kraken WebSocket v2 API format for real-time market data streaming.
//!
//! # Key Types
//!
//! - [`Response`] - Main response enum for all Kraken WebSocket messages
//! - [`DEFAULT_URL`] - Default WebSocket endpoint for Kraken API

pub use channel::ChannelResponse;
pub use channel::ticker::TickerResponse;
use serde::{Deserialize, Serialize};

use crate::api::types::message::PublicMessageResponse;

pub mod channel;
pub mod message;

/// The default URL for the Kraken WebSocket API.
pub const DEFAULT_URL: &str = "wss://ws.kraken.com/v2";

/// Represents the different types of responses from the Kraken WebSocket API.
///
/// The `Response` enum can represent various types of messages from the Kraken WebSocket API,
/// including public message responses, channel data updates, and ping messages.
/// Each variant corresponds to a specific type of message, allowing for flexible handling
/// of various response types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Response {
    /// A response for public messages (e.g., subscribe/unsubscribe confirmations).
    ///
    /// This variant contains a confirmation response for public control messages
    /// (e.g., subscribe/unsubscribe).
    PublicMessage(PublicMessageResponse),

    /// A response containing data from a subscribed channel.
    ///
    /// This variant contains actual market data updates from subscribed channels,
    /// such as ticker information for trading pairs.
    Channel(ChannelResponse),

    /// A ping message for connection keep-alive.
    ///
    /// This variant represents ping messages sent by the Kraken API to maintain
    /// the WebSocket connection.
    Ping,
}
