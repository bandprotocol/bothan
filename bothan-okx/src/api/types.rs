//! Types for OKX WebSocket API interaction.
//!
//! This module provides types for deserializing events and responses from the OKX WebSocket API,
//! including subscription responses, data updates, and ping messages. The module supports
//! the OKX WebSocket v5 API format for real-time market data streaming.
//!
//! # Key Types
//!
//! - [`Response`] - Main response enum for all OKX WebSocket messages
//! - [`DEFAULT_URL`] - Default WebSocket endpoint for OKX API

pub use channel::{ChannelArgument, PushData};
use serde::{Deserialize, Serialize};

use crate::api::types::ticker::Ticker;

pub mod channel;
pub mod subscription;
pub mod ticker;

/// The default URL for the OKX WebSocket API.
pub const DEFAULT_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

/// Represents the different types of responses from the OKX WebSocket API.
///
/// The `Response` enum can represent various types of messages from the OKX WebSocket API,
/// including subscription confirmations, ticker data updates, and ping messages.
/// Each variant corresponds to a specific type of message, allowing for flexible handling
/// of various response types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Response {
    /// A response from a WebSocket subscription message.
    ///
    /// This variant contains subscription-related responses from the OKX API,
    /// including subscription confirmations and error messages.
    TickerSubscription(subscription::Response<ticker::Request>),

    /// A response containing data from a subscribed channel.
    ///
    /// This variant contains actual market data updates from subscribed channels,
    /// such as ticker information for trading pairs.
    TickersChannel(PushData<Vec<Ticker>>),

    /// A ping message for connection keep-alive.
    ///
    /// This variant represents ping messages sent by the OKX API to maintain
    /// the WebSocket connection.
    Ping,
}
