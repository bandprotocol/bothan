//! Error types for OKX API interactions.
//!
//! This module defines error types used throughout the OKX integration for handling
//! various error conditions that can occur during WebSocket communication and data processing.
//!
//! The module provides two main error types:
//!
//! - [`Error`] - General errors that can occur during WebSocket operations
//! - [`ListeningError`] - Errors specific to the listening/processing phase

use std::io;

/// General errors that can occur during OKX WebSocket operations.
///
/// This enum represents various error conditions that can arise when communicating
/// with the OKX WebSocket API, including I/O errors, parsing errors, and unsupported
/// message types.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error occurred during WebSocket communication.
    ///
    /// This variant wraps standard I/O errors that can occur when reading from
    /// or writing to the WebSocket connection.
    #[error("failed to read message")]
    Io(#[from] io::Error),

    /// JSON parsing error occurred when deserializing a message.
    ///
    /// This variant wraps serde JSON errors that can occur when parsing
    /// WebSocket messages from the OKX API.
    #[error("failed to parse message")]
    ParseError(#[from] serde_json::Error),

    /// Received an unsupported WebSocket message type.
    ///
    /// This variant indicates that the WebSocket connection received a message
    /// type that is not supported by the OKX integration.
    #[error("unsupported message")]
    UnsupportedWebsocketMessageType,
}

/// Errors that can occur during the listening and processing phase.
///
/// This enum represents errors that can arise when processing incoming WebSocket
/// messages and converting them to asset information. It includes errors from
/// the underlying WebSocket communication as well as data validation errors.
#[derive(Debug, thiserror::Error)]
pub enum ListeningError {
    /// Error from the underlying WebSocket communication.
    ///
    /// This variant wraps errors from the general [`Error`] type that can occur
    /// during WebSocket operations.
    #[error(transparent)]
    Error(#[from] Error),

    /// Invalid price data encountered during processing.
    ///
    /// This variant indicates that the price data received from the OKX API
    /// could not be converted to a valid decimal value.
    #[error(transparent)]
    InvalidPrice(#[from] rust_decimal::Error),

    /// Invalid timestamp data encountered during processing.
    ///
    /// This variant indicates that the timestamp data received from the OKX API
    /// could not be parsed as a valid integer.
    #[error(transparent)]
    InvalidTimestamp(#[from] std::num::ParseIntError),
}
