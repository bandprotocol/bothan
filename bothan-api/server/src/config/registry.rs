use serde::Deserialize;

/// A local registry source configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct LocalRegistry {
    pub path: String,
}

/// A IPFS registry source configuration.
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
