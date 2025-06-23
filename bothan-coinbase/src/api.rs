//! Coinbase WebSocket API client implementation.
//!
//! This module provides types and utilities for interacting with the Coinbase WebSocket API,
//! including configuration, connection management, error handling, and message processing.
//!
//! The module provides:
//!
//! - [`error`] — Custom error types used during WebSocket client configuration and message processing.
//! - [`types`] — Type definitions used for communication with the Coinbase WebSocket API.
//! - [`websocket`] — WebSocket client implementation for real-time data streaming.
//!
//! # Integration with Workers
//!
//! This module is intended to be used by worker implementations that subscribe to Coinbase WebSocket streams for real-time updates.
//! The WebSocket client can be used for streaming market data, such as trades, order book updates, and price changes.
//!
//! The module exports [`WebSocketConnection`] and [`WebSocketConnector`] for WebSocket-based communication.

pub use types::channels::ticker::Ticker;
pub use websocket::{WebSocketConnection, WebSocketConnector};

pub mod error;
pub mod types;
pub mod websocket;
