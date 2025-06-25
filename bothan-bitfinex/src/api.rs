//! Bitfinex REST API client implementation.
//!
//! This module provides types and utilities for interacting with the Bitfinex REST API,
//! including configuration, connection management, error handling, and message processing.
//!
//! The module provides:
//!
//! - [`builder`] — Builder pattern for creating configured REST API clients.
//! - [`error`] — Custom error types used during REST API client configuration and data processing.
//! - [`msg`] — Message types used for communication with the Bitfinex REST API.
//! - [`rest`] — REST API client implementation for fetching market data.
//!
//! # Integration with Workers
//!
//! This module is intended to be used by worker implementations that poll the Bitfinex REST API
//! for real-time updates. The REST client can be used for fetching market data, such as ticker
//! information for both spot and funding markets.
//!
//! The module exports [`RestApi`] and [`RestApiBuilder`] for REST-based communication.

pub mod builder;
pub mod error;
pub mod msg;
pub mod rest;
