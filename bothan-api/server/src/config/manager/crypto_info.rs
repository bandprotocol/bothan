use serde::Deserialize;

use crate::config::manager::crypto_info::registry::RegistrySeedConfig;
use crate::config::manager::crypto_info::sources::CryptoSourceConfigs;

pub mod registry;
pub mod sources;

/// The configuration for bothan-api's crypto asset info manager.
#[derive(Clone, Debug, Deserialize)]
pub struct CryptoInfoManagerConfig {
    /// The registry source for the crypto asset info manager.
    pub registry: RegistrySeedConfig,
    /// The source configuration for the crypto asset info manager.
    pub source: CryptoSourceConfigs,
    /// The stale threshold for the crypto asset info.
    pub stale_threshold: i64,
    /// Flag to allow for registry updates through the api
    pub no_update: bool,
}
