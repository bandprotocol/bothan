//! Configuration options for initializing a `KrakenWorker`.
//!
//! This module provides the [`WorkerOpts`] used to configure a `KrakenWorker`.
//! It allows setting the WebSocket API URL, which is used by [`Worker`](`crate::worker::Worker`)
//! to interact with the Kraken WebSocket API.
//!
//! The module provides:
//! - The [`WorkerOpts`] for specifying worker parameters
//! - Serialization and deserialization support for configuration files
//! - Defaults for WebSocket API URL
//! - Internal helpers for handling empty or missing configuration values

use serde::{Deserialize, Serialize};

use crate::api::types::DEFAULT_URL;

/// Options for configuring the `KrakenWorker`.
///
/// [`WorkerOpts`] provides a way to specify custom values for creating a
/// `KrakenWorker`. It specifies parameters such as the WebSocket API URL,
/// which is used to interact with the Kraken WebSocket API.
///
/// # Examples
///
/// ```rust
/// use bothan_kraken::worker::opts::WorkerOpts;
///
/// let opts = WorkerOpts {
///     url: "wss://ws.kraken.com/v2".to_string(),
/// };
///
/// // Or use defaults
/// let opts = WorkerOpts::default();
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    /// The base URL for the worker's WebSocket API connection. If not provided,
    /// a default URL will be used.
    #[serde(default = "default_url")]
    pub url: String,
}

/// This function returns the default WebSocket API URL for the Kraken API.
fn default_url() -> String {
    DEFAULT_URL.to_string()
}

impl Default for WorkerOpts {
    /// Creates a new `WorkerOpts` with default values.
    ///
    /// This method initializes the configuration with:
    /// - Default Kraken WebSocket API URL
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_kraken::worker::opts::WorkerOpts;
    ///
    /// let opts = WorkerOpts::default();
    /// assert_eq!(opts.url, "wss://ws.kraken.com/v2");
    /// ```
    fn default() -> Self {
        Self { url: default_url() }
    }
}
