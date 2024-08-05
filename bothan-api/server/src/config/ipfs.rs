use serde::{Deserialize, Serialize};

pub const DEFAULT_IPFS_ENDPOINT: &str = "https://ipfs.io";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum IpfsAuthentication {
    #[default]
    None,
    Header {
        key: String,
        value: String,
    },
}

/// A IPFS registry source configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IpfsConfig {
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default)]
    pub authentication: IpfsAuthentication,
}

fn default_endpoint() -> String {
    DEFAULT_IPFS_ENDPOINT.to_string()
}

impl Default for IpfsConfig {
    fn default() -> Self {
        IpfsConfig {
            endpoint: default_endpoint(),
            authentication: IpfsAuthentication::default(),
        }
    }
}
