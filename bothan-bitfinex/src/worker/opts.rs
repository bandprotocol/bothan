//! Configuration options for initializing a `BitfinexWorker`.
//!
//! This module provides the [`WorkerOpts`] used to configure a `BitfinexWorker`.
//! It allows setting the REST API URL and the update interval for polling,
//! which are used by [`Worker`](`crate::worker::Worker`) to interact with the Bitfinex REST API.
//!
//! The module provides:
//! - The [`WorkerOpts`] for specifying worker parameters
//! - Serialization and deserialization support for configuration files
//! - Defaults for REST API URL and update interval
//! - Internal helpers for handling empty or missing configuration values
//!
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::api::rest::DEFAULT_URL;

const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/// Options for configuring the `BitfinexWorker`.
///
/// [`WorkerOpts`] provides a way to specify custom values for creating a
/// `BitfinexWorker`. It specifies parameters such as the REST API URL and the update interval
/// for polling, which are used to interact with the Bitfinex REST API.
///
/// # Examples
///
/// ```rust
/// use bothan_bitfinex::worker::opts::WorkerOpts;
/// use std::time::Duration;
///
/// let opts = WorkerOpts {
///     url: "https://api-pub.bitfinex.com/v2/".to_string(),
///     update_interval: Duration::from_secs(30),
/// };
///
/// // Or use defaults
/// let opts = WorkerOpts::default();
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    /// The base URL for the worker's REST API connection. If not provided,
    /// a default URL will be used.
    #[serde(default = "default_url")]
    pub url: String,

    /// The interval between REST API polling requests for asset updates.
    /// If not specified, a default value will be used.
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
}

/// This function returns the default REST API URL for the Bitfinex API.
fn default_url() -> String {
    DEFAULT_URL.to_string()
}

/// This function returns the default update interval for REST API polling.
fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

impl Default for WorkerOpts {
    /// Creates a new `WorkerOpts` with default values.
    ///
    /// This method initializes the configuration with:
    /// - Default Bitfinex REST API URL
    /// - Default update interval for polling
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_bitfinex::worker::opts::WorkerOpts;
    ///
    /// let opts = WorkerOpts::default();
    /// assert_eq!(opts.url, "https://api-pub.bitfinex.com/v2/");
    /// assert_eq!(opts.update_interval.as_secs(), 60);
    /// ```
    fn default() -> Self {
        Self {
            url: default_url(),
            update_interval: default_update_interval(),
        }
    }
}
