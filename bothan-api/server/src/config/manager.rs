use crypto_info::CryptoInfoManagerConfig;
use serde::{Deserialize, Serialize};

pub mod crypto_info;

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ManagerConfig {
    pub crypto: CryptoInfoManagerConfig,
}
