use serde::Deserialize;

/// The configuration for the registry.
#[derive(Clone, Debug, Deserialize)]
pub struct LocalRegistry {
    pub path: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IpfsRegistry {
    pub hash: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RegistrySource {
    Local(LocalRegistry),
    Ipfs(IpfsRegistry),
}

/// The registry source configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct RegistryConfig {
    pub crypto_assets: RegistrySource,
}
