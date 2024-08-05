use serde::{Deserialize, Serialize};

use crate::config::manager::crypto_info::sources::CryptoSourceConfigs;

pub mod registry;
pub mod sources;

/// The configuration for bothan-api's crypto asset info manager.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CryptoInfoManagerConfig {
    /// The source configuration for the crypto asset info manager.
    pub source: CryptoSourceConfigs,
    /// The stale threshold for the crypto asset info.
    #[serde(default = "default_stale_threshold")]
    pub stale_threshold: i64,
}

fn default_stale_threshold() -> i64 {
    300
}

impl Default for CryptoInfoManagerConfig {
    fn default() -> Self {
        CryptoInfoManagerConfig {
            source: CryptoSourceConfigs::default(),
            stale_threshold: default_stale_threshold(),
        }
    }
}
