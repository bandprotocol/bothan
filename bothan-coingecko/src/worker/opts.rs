//! Configuration options for initializing a `CoinGeckoWorker`.
//!
//! This module provides the [`WorkerOpts`] used to configure a `CoinGeckoWorker`.
//! It allows setting the API endpoint, authentication, user agent,
//! and polling interval used by [`Worker`](`crate::worker::Worker`) to fetch data from the CoinGecko REST API.
//!
//! The module provides:
//! - The [`WorkerOpts`] for specifying worker parameters
//! - Serialization and deserialization support for configuration files
//! - Defaults for user agent and update interval
//! - Internal helpers for handling empty or missing configuration values
//!
use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api::types::{DEFAULT_URL, DEFAULT_USER_AGENT};

const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/// Options for configuring the `CoinGeckoWorker`.
///
/// [`WorkerOpts`] provides a way to specify custom values for creating a
/// `CoinGeckoWorker`. It specifies parameters such as the API endpoint URL, an optional API key,
/// a custom user agent, and the polling interval for fetching data.
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use bothan_coingecko::worker::opts::WorkerOpts;
///
/// let opts = WorkerOpts {
///     url: Some("https://api.coingecko.com/api/v3".to_string()),
///     api_key: Some("my-secret-key".to_string()),
///     user_agent: "my-custom-agent/1.0".to_string(),
///     update_interval: Duration::from_secs(30),
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    /// Optional base URL for the CoinGecko REST API. If not provided,
    /// the default public URL will be used.
    #[serde(serialize_with = "none_is_default_url")]
    pub url: Option<String>,
    /// Optional API key for CoinGecko Pro. Can be an empty string in configs,
    /// which will be interpreted as `None`.
    #[serde(default)]
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(serialize_with = "none_is_empty_string")]
    pub api_key: Option<String>,
    /// User-Agent HTTP header.
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    /// Duration between API polling.
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
}

/// This function returns the default user agent.
fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_string()
}

/// This function returns the default update interval duration.
fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

impl Default for WorkerOpts {
    /// Creates a new `WorkerOpts` with default values.
    ///
    /// This method initializes the configuration with:
    /// - Default CoinGecko REST API URL
    /// - No API key
    /// - Default user agent
    /// - Default update interval
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings
    fn default() -> Self {
        Self {
            url: None,
            api_key: None,
            user_agent: default_user_agent(),
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

/// Serializer helper: converts `None` to the default CoinGecko REST API URL.
///
/// This function is useful when serializing configuration data where a missing value (`None`)
/// should be represented as the default public CoinGecko REST API URL (`DEFAULT_URL`) in the output format, such as JSON or YAML.
fn none_is_default_url<S: Serializer>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(val) => serializer.serialize_str(val),
        None => serializer.serialize_str(DEFAULT_URL),
    }
}
