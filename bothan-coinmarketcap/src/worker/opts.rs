//! Configuration options for initializing a `CoinMarketCapWorker`.
//!
//! This module provides the [`WorkerOpts`] used to configure a `CoinMarketCapWorker`.
//! It allows setting the API endpoint, authentication, and polling interval used by [`Worker`](`crate::worker::Worker`) to fetch data from the CoinMarketCap REST API.
//!
//! The module provides:
//! - The [`WorkerOpts`] for specifying worker parameters
//! - Serialization and deserialization support for configuration files
//! - Defaults for update interval
//! - Internal helpers for handling empty or missing configuration values

use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api::types::DEFAULT_URL;
use crate::worker::types::DEFAULT_UPDATE_INTERVAL;

/// Options for configuring the `CoinMarketCapWorker`.
///
/// [`WorkerOpts`] provides a way to specify custom values for creating a
/// `CoinMarketCapWorker`. It specifies parameters such as the API endpoint URL, an optional API key,
/// and the polling interval for fetching data.
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use bothan_coinmarketcap::worker::opts::WorkerOpts;
///
/// let opts = WorkerOpts {
///     url: "https://pro-api.coinmarketcap.com".to_string(),
///     api_key: Some("my-secret-key".to_string()),
///     update_interval: Duration::from_secs(30),
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    /// The URL for the CoinMarketCap REST API.
    /// If none is provided, the CoinMarketCap Pro API base URL will be used.
    #[serde(default = "default_url")]
    pub url: String,
    /// Optional API key for CoinMarketCap.
    #[serde(default)]
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(serialize_with = "none_is_empty_string")]
    pub api_key: Option<String>,
    /// Duration between API polling.
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
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
    /// - Default CoinMarketCap Pro API URL
    /// - No API key
    /// - Default update interval
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings
    fn default() -> Self {
        Self {
            url: default_url(),
            api_key: None,
            update_interval: default_update_interval(),
        }
    }
}

/// Deserializer helper: converts empty strings to `None`.
///
/// This function is useful when parsing configuration files where empty strings or missing values
/// should be interpreted as `None`.
fn empty_string_is_none<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

/// Serializer helper: converts `None` to an empty string.
///
/// This function is useful when serializing configuration data where a missing value (`None`)
/// should be represented as an empty string (`""`) in the output format, such as JSON or YAML.
fn none_is_empty_string<S: Serializer>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(val) => serializer.serialize_str(val),
        None => serializer.serialize_str(""),
    }
}
