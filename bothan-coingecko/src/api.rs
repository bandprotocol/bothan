//! CoinGecko REST API client implementation.
//!
//! This module provides types and utilities for interacting with the CoinGecko REST API,
//! including configuration, request execution, error handling, and response deserialization.
//!
//! The module provides:
//!
//! - [`builder`] — A builder pattern for creating [`RestApi`] clients with optional parameters like base URL, user agent, and API key.
//! - [`rest`] — Core API client implementation, including HTTP request logic and integration with Bothan’s `AssetInfoProvider` trait.
//! - [`types`] — Data types that represent CoinGecko REST API responses such as [`Coin`](types::Coin) and [`Price`](types::Price).
//! - [`error`] — Custom error types used during API client configuration and request processing.
//!
//! # Integration with Workers
//!
//! This module is intended to be used by worker implementations (such as [`Worker`](`crate::worker::Worker`))
//! that periodically query CoinGecko for asset data. The [`RestApi`] implements the
//! [`AssetInfoProvider`](bothan_lib::worker::rest::AssetInfoProvider) trait, which allows
//! CoinGecko responses to be translated into Bothan-compatible asset updates.

pub use builder::RestApiBuilder;
pub use rest::RestApi;

pub mod builder;
pub mod error;
pub mod rest;
pub mod types;
