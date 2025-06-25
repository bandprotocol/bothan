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

pub mod builder;
pub mod error;
pub mod msg;
pub mod rest;
