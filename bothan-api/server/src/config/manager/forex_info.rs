//! Bothan API server forex info manager configuration.
//!
//! Settings for forex asset info sources and staleness threshold.

use serde::{Deserialize, Serialize};

use crate::config::manager::forex_info::sources::ForexSourceConfigs;

/// Forex info source configuration module.
pub mod sources;

/// Configuration for the Bothan API Server's forex asset info manager.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForexInfoManagerConfig {
    /// The source configuration for the forex asset info manager.
    pub source: ForexSourceConfigs,
    /// The stale threshold for the forex asset info (in seconds).
    /// Any source that has not been updated in this amount of time
    /// relative to the call will be considered stale.
    #[serde(default = "default_stale_threshold")]
    pub stale_threshold: i64,
}

/// Returns the default stale threshold (in seconds).
fn default_stale_threshold() -> i64 {
    3600
}

impl Default for ForexInfoManagerConfig {
    /// Creates a new `ForexInfoManagerConfig` with default values.
    fn default() -> Self {
        ForexInfoManagerConfig {
            source: ForexSourceConfigs::default(),
            stale_threshold: default_stale_threshold(),
        }
    }
}
