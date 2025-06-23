//! Error types for Kraken API interactions.
//!
//! This module defines error types used throughout the Kraken integration for handling
//! various error conditions that can occur during WebSocket communication and data processing.
//!
//! The module provides two main error types:
//!
//! - [`Error`] - General errors that can occur during WebSocket operations
//! - [`ListeningError`] - Errors specific to the listening/processing phase
//!
use std::io;

/// General errors that can occur during Kraken WebSocket operations.
///
/// This enum represents various error conditions that can arise when communicating
/// with the Kraken WebSocket API, including I/O errors, parsing errors, and unsupported
/// message types.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Indicates a failure to read WebSocket messages.
    #[error("failed to read message")]
    IO(#[from] io::Error),

    /// Indicates a failure to parse a WebSocket message.
    #[error("failed to parse message")]
    ParseError(#[from] serde_json::Error),

    /// Indicates that the WebSocket message type is not supported.
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
    /// Indicates an error while processing WebSocket messages.
    #[error(transparent)]
    Error(#[from] Error),

    /// Indicates that the received channel ID is invalid or malformed.
    #[error("received invalid channel id")]
    InvalidChannelId,

    /// Indicates that the received price data contains NaN values.
    #[error("received NaN")]
    InvalidPrice,
}
