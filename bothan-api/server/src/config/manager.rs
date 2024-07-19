use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Crypto {
    pub stale_threshold: i64,
    pub no_update: bool,
}

/// The configuration for the manager.
#[derive(Clone, Debug, Deserialize)]
pub struct ManagerConfig {
    pub crypto: Crypto,
}
