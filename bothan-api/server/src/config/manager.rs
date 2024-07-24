use serde::Deserialize;

use crypto_info::CryptoInfoManagerConfig;

pub mod crypto_info;

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Deserialize)]
pub struct ManagerConfig {
    pub crypto: CryptoInfoManagerConfig,
}
