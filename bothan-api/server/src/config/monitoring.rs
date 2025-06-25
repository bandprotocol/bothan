//! Bothan API server monitoring configuration.
//!
//! Settings for monitoring endpoint, key path, and enable flag.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::monitoring::MonitoringConfig;
//! let config = MonitoringConfig::default();
//! ```
//!
//! ## Configuration Example
//!
//! ```toml
//! [monitoring]
//! endpoint = "https://bothan-monitoring.bandchain.org"
//! path = "/home/user/.bothan/keyring/broadcaster.info"
//! enabled = true
//! ```

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Configuration for the Bothan API Server's monitoring service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// The monitoring endpoint URL.
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    /// The path to where the key for the monitoring service is stored.
    #[serde(default = "default_path")]
    pub path: PathBuf,
    /// Whether monitoring is enabled.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

/// Returns the default monitoring endpoint URL.
fn default_endpoint() -> String {
    "https://bothan-monitoring.bandchain.org".to_string()
}

/// Returns the default path for the monitoring key file.
fn default_path() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan/keyring/broadcaster.info")
}

/// Returns whether monitoring is enabled by default.
fn default_enabled() -> bool {
    true
}

impl Default for MonitoringConfig {
    /// Creates a new `MonitoringConfig` with default values.
    fn default() -> Self {
        MonitoringConfig {
            endpoint: default_endpoint(),
            path: default_path(),
            enabled: default_enabled(),
        }
    }
}
