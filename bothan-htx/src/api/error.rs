//! Error types for HTX API client operations.
//!
//! This module provides custom error types used throughout the HTX API integration,
//! particularly for handling websocket messages, data parsing, and connection management.

use std::io;

use tokio_tungstenite::tungstenite;

/// Errors related to HTX API client operations.
///
/// These errors can occur during communication with the HTX WebSocket API,
/// including message parsing, I/O operations, and unsupported message types.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Indicates a failure to read WebSocket messages.
    #[error("failed to read message")]
    Io(#[from] io::Error),

    /// Indicates a failure to parse a WebSocket message.
    #[error("failed to parse message: {msg}")]
    ParseError {
        #[source]
        source: serde_json::Error,
        msg: String,
    },

    /// Indicates that the WebSocket message type is not supported.
    #[error("unsupported message: {0}")]
    UnsupportedWebsocketMessageType(String),
}

/// Errors encountered while listening for HTX API events.
///
/// These errors can occur during subscription to asset updates, message processing,
/// or when handling ping/pong messages from the HTX WebSocket stream.
#[derive(Debug, thiserror::Error)]
pub enum ListeningError {
    /// Indicates an error while processing WebSocket messages.
    #[error(transparent)]
    Error(#[from] Error),

    /// Indicates that the received channel ID is invalid or malformed.
    #[error("received invalid channel id: {0}")]
    InvalidChannelId(String),

    /// Indicates that the received price data contains invalid values.
    #[error("invalid price value {price} for symbol {symbol}")]
    InvalidPrice { symbol: String, price: f64 },

    /// Indicates a failure to send a pong response to a ping message.
    #[error("failed to pong")]
    PongFailed(#[from] tungstenite::Error),
}
