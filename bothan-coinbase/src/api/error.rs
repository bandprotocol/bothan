//! Error types for Coinbase API client operations.
//!
//! This module provides custom error types used throughout the Coinbase API integration,
//! particularly for handling websocket messages and price validation errors.

/// Errors related to Coinbase API client operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Indicates a failure to parse a WebSocket message.
    #[error("failed to parse message")]
    ParseError(#[from] serde_json::Error),

    /// Indicates that the WebSocket message type is not supported.
    #[error("unsupported message")]
    UnsupportedWebsocketMessageType,
}

/// Errors encountered while listening for Coinbase API events.
///
/// These errors can occur during subscription to asset updates or when processing
/// incoming messages from the Coinbase WebSocket stream.
#[derive(Debug, thiserror::Error)]
pub enum ListeningError {
    /// Indicates an error while subscribing to a specific asset stream.
    #[error(transparent)]
    Error(#[from] Error),

    /// Indicates an error while parsing a message from the WebSocket stream.
    #[error(transparent)]
    InvalidPrice(#[from] rust_decimal::Error),

    /// Indicates an error while parsing a timestamp from the WebSocket message.
    #[error(transparent)]
    InvalidTimestamp(#[from] chrono::ParseError),
}
