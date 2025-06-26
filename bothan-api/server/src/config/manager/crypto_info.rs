//! Bothan API server crypto info manager configuration.
//!
//! Settings for crypto asset info sources and staleness threshold.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::manager::crypto_info::CryptoInfoManagerConfig;
//! let config = CryptoInfoManagerConfig::default();
//! ```

use serde::{Deserialize, Serialize};

use crate::config::manager::crypto_info::sources::CryptoSourceConfigs;

/// Crypto info source configuration module.
pub mod sources;

/// Configuration for the Bothan API Server's crypto asset info manager.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoInfoManagerConfig {
    /// The source configuration for the crypto asset info manager.
    pub source: CryptoSourceConfigs,
    /// The stale threshold for the crypto asset info (in seconds).
    /// Any source that has not been updated in this amount of time
    /// relative to the call will be considered stale.
    #[serde(default = "default_stale_threshold")]
    pub stale_threshold: i64,
}

/// Returns the default stale threshold (in seconds).
fn default_stale_threshold() -> i64 {
    300
}

impl Default for CryptoInfoManagerConfig {
    /// Creates a new `CryptoInfoManagerConfig` with default values.
    fn default() -> Self {
        CryptoInfoManagerConfig {
            source: CryptoSourceConfigs::default(),
            stale_threshold: default_stale_threshold(),
        }
    }
}
