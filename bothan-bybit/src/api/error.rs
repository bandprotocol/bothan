//! Error types for Bybit API client operations.
//!
//! This module provides custom error types used throughout the Bybit API integration,
//! particularly for handling WebSocket messages and price validation errors.

/// Errors related to Bybit API client operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to parse a message from the WebSocket.
    #[error("failed to parse message: {msg}")]
    ParseError {
        #[source]
        source: serde_json::Error,
        msg: String,
    },

    /// Received an unsupported message type from the WebSocket.
    #[error("unsupported message: {0}")]
    UnsupportedWebsocketMessageType(String),
}

/// Errors that can occur while listening for Bybit WebSocket events.
///
/// These errors can occur during subscription to asset updates or when processing
/// incoming messages from the Bybit WebSocket stream.
#[derive(Debug, thiserror::Error)]
pub enum ListeningError {
    /// An error occurred while processing a WebSocket message.
    #[error(transparent)]
    Error(#[from] Error),

    /// An invalid price was encountered while parsing a message.
    #[error("invalid price value {price} for symbol {symbol}")]
    InvalidPrice {
        #[source]
        source: rust_decimal::Error,
        symbol: String,
        price: String,
    },
}
