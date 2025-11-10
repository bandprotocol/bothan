//! Configuration options for initializing a `BandWorker`.
//!
//! This module provides the [`WorkerOpts`] used to configure a `BandWorker`.
//! It allows setting the API endpoint and polling interval used by [`Worker`](`crate::worker::Worker`) to fetch data from the Band REST API.
//!
//! The module provides:
//! - The [`WorkerOpts`] for specifying worker parameters
//! - Serialization and deserialization support for configuration files
//! - Defaults for update interval
use std::time::Duration;

use serde::{Deserialize, Serialize};

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
/// use std::time::Duration;
///
/// let opts = WorkerOpts {
///     name: "band",
///     url: "https://bandsource-url.com".to_string(),
///     update_interval: Duration::from_secs(30),
/// };
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkerOpts {
    #[serde(skip)]
    name: &'static str,
    /// The URL for the Band REST API.
    pub url: String,
    /// Duration between API polling.
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
}

/// This function returns the default update interval duration.
fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

impl WorkerOpts {
    /// Creates a new `WorkerOpts` with default values.
    ///
    /// This method initializes the configuration with:
    /// - Band worker name (must be provided by the caller)
    /// - Band API URL (must be provided by the caller)
    /// - Default update interval
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings
    pub fn new(name: &'static str, url: &str, update_interval: Option<Duration>) -> Self {
        Self {
            name,
            url: url.to_string(),
            update_interval: update_interval.unwrap_or(default_update_interval()),
        }
    }

    /// Returns the name identifier for the worker.
    pub fn name(&self) -> &'static str { self.name }
}
