//! Error types for Binance API client operations.
//!
//! This module provides custom error types used throughout the Binance API integration,
//! particularly for handling websocket messages and price validation errors.

/// Errors related to Binance API client operations.
///
/// Errors related to communication with the Binance API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Indicates a failure to parse a websocket message.
    #[error("failed to parse message: {msg}")]
    ParseError {
        #[source]
        source: serde_json::Error,
        msg: String,
    },

    /// Indicates that the websocket message type is not supported.
    #[error("unsupported message: {0}")]
    UnsupportedWebsocketMessageType(String),
}

/// Errors encountered while listening for Binance API events.
///
/// These errors can occur during subscription to asset updates or when processing
/// incoming messages from the Binance WebSocket stream.
#[derive(Debug, thiserror::Error)]
pub enum ListeningError {
    /// Indicates an error while subscribing to a specific asset stream.
    #[error(transparent)]
    Error(#[from] Error),

    /// Indicates an error while parsing a message from the WebSocket stream.
    #[error("invalid price value {price} for symbol {symbol}")]
    InvalidPrice {
        #[source]
        source: rust_decimal::Error,
        symbol: String,
        price: String,
    },
}
