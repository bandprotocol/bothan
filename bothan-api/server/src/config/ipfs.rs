use serde::Deserialize;

/// The configuration for bothan-api's IPFS client.
#[derive(Clone, Debug, Deserialize)]
pub struct IpfsConfig {
    /// The URL of the IPFS node.
    pub url: String,
}
