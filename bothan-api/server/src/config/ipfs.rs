use serde::Deserialize;

pub const DEFAULT_IPFS_ENDPOINT: &str = "https://ipfs.io";

#[derive(Clone, Debug, Deserialize, Default)]
pub enum IpfsAuthentication {
    #[default]
    None,
    Bearer {
        token: String,
    },
    Basic {
        username: String,
        password: String,
    },
}

/// A IPFS registry source configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct IpfsConfig {
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default)]
    pub authentication: IpfsAuthentication,
}

fn default_endpoint() -> String {
    DEFAULT_IPFS_ENDPOINT.to_string()
}
