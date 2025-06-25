//! HTX WebSocket API client implementation.
//!
//! This module provides types and utilities for interacting with the HTX WebSocket API,
//! including configuration, connection management, error handling, and message processing.
//!
//! The module provides:
//!
//! - [`error`] — Custom error types used during WebSocket client configuration and message processing.
//! - [`types`] — Message types used for communication with the HTX WebSocket API.
//! - [`websocket`] — WebSocket client implementation for real-time data streaming.
//!
//! # Integration with Workers
//!
//! This module is intended to be used by worker implementations that subscribe to HTX WebSocket streams for real-time updates.
//! The WebSocket client can be used for streaming market data, such as ticker updates and price changes.
//!
//! The module exports [`WebSocketConnection`] and [`WebSocketConnector`] for WebSocket-based communication.

pub use websocket::{WebSocketConnection, WebSocketConnector};

pub mod error;
pub mod types;
pub mod websocket;
