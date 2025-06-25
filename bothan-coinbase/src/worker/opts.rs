//! Configuration options for initializing a `CoinbaseWorker`.
//!
//! This module provides the [`WorkerOpts`] used to configure a `CoinbaseWorker`.
//! It allows setting the WebSocket URL and the maximum number of subscriptions per connection,
//! which are used by [`Worker`](`crate::worker::Worker`) to interact with the Coinbase WebSocket API.
//!
//! The module provides:
//! - The [`WorkerOpts`] for specifying worker parameters
//! - Serialization and deserialization support for configuration files
//! - Defaults for WebSocket URL and maximum subscriptions per connection
//! - Internal helpers for handling empty or missing configuration values
//!
use serde::{Deserialize, Serialize};

use crate::api::types::DEFAULT_URL;
use crate::worker::MAX_SUBSCRIPTION_PER_CONNECTION;

/// Options for configuring the `CoinbaseWorkerBuilder`.
///
/// `CoinbaseWorkerBuilderOpts` provides a way to specify custom settings for creating a
/// `CoinbaseWorker`. This struct allows users to set optional parameters such as the WebSocket URL
/// and the internal channel size, which will be used during the construction of the
/// `CoinbaseWorker`.
///
/// # Examples
///
/// ```rust
/// use bothan_coinbase::worker::opts::WorkerOpts;  
/// let opts = WorkerOpts {
///     url: "wss://ws-feed.exchange.coinbase.com".to_string(),
///     max_subscription_per_connection: 100,
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    /// The base URL for the worker's connection. If not provided,
    /// a default URL will be used.
    #[serde(default = "default_url")]
    pub url: String,

    /// The maximum number of subscriptions allowed per connection.
    /// If not specified, a default value will be used.
    #[serde(default = "default_max_subscription_per_connection")]
    pub max_subscription_per_connection: usize,
}

/// This function returns the default WebSocket URL for the Coinbase API.
fn default_url() -> String {
    DEFAULT_URL.to_string()
}

/// This function returns the default maximum number of subscriptions per connection.
fn default_max_subscription_per_connection() -> usize {
    MAX_SUBSCRIPTION_PER_CONNECTION
}

impl Default for WorkerOpts {
    /// Creates a new `WorkerOpts` with default values.
    ///
    /// This method initializes the configuration with:
    /// - Default Coinbase WebSocket URL
    /// - Default maximum number of subscriptions per connection
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings.
    fn default() -> Self {
        Self {
            url: default_url(),
            max_subscription_per_connection: default_max_subscription_per_connection(),
        }
    }
}
