//! Configuration options for initializing a `BandWorker`.
//!
//! This module provides the [`WorkerOpts`] used to configure a `BandWorker`.
//! It allows setting the API endpoint, authentication, and polling interval used by [`Worker`](`crate::worker::Worker`) to fetch data from the Band REST API.
//!
//! The module provides:
//! - The [`WorkerOpts`] for specifying worker parameters
//! - Serialization and deserialization support for configuration files
//! - Defaults for update interval
//! - Internal helpers for handling empty or missing configuration values
use std::time::Duration;

use serde::{Deserialize, Serialize};

const DEFAULT_NAME: &str = "band";

use crate::api::types::DEFAULT_URL;

const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/// Options for configuring the `BandWorker`.
///
/// [`WorkerOpts`] provides a way to specify custom values for creating a
/// `BandWorker`. It specifies parameters such as the API endpoint URL,
/// and the polling interval for fetching data.
///
/// # Examples
///
/// ```rust
/// use bothan_band::worker::opts::WorkerOpts;
///
/// let opts = WorkerOpts {
///     url: "https://bandsource-url.com".to_string(),
///     update_interval: Duration::from_secs(30),
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    #[serde(default = "default_name")]
    pub name: String,
    /// The URL for the Band REST API.
    /// If none is provided, the Band Pro API base URL will be used.
    #[serde(default = "default_url")]
    pub url: String,
    /// Duration between API polling.
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
}

/// This function returns the default name for the worker.
fn default_name() -> String {
    DEFAULT_NAME.to_string()
}

/// This function returns the default url.
fn default_url() -> String {
    DEFAULT_URL.to_string()
}

/// This function returns the default update interval duration.
fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

impl Default for WorkerOpts {
    /// Creates a new `WorkerOpts` with default values.
    ///
    /// This method initializes the configuration with:
    /// - Default name (must change if multiple workers are used)
    /// - Default Band API URL
    /// - Default update interval
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings
    fn default() -> Self {
        Self {
            name: default_name(),
            url: default_url(),
            update_interval: default_update_interval(),
        }
    }
}
