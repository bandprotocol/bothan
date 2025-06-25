//! Bothan API server IPFS configuration.
//!
//! Settings for IPFS endpoint, authentication, and timeout.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::ipfs::{IpfsConfig, IpfsAuthentication};
//!
//! // Use default configuration
//! let config = IpfsConfig::default();
//!
//! // Custom configuration
//! let config = IpfsConfig {
//!     endpoint: "https://my-ipfs-endpoint.com".to_string(),
//!     authentication: IpfsAuthentication::None,
//!     timeout: Some(std::time::Duration::from_secs(10)),
//! };
//! ```
//!
//! ## Configuration Example
//!
//! ```toml
//! [ipfs]
//! endpoint = "https://ipfs.io"
//! # authentication = { Header = { key = "Authorization", value = "Bearer ..." } }
//! # timeout = 10
//! ```

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// The default IPFS endpoint URL.
pub const DEFAULT_IPFS_ENDPOINT: &str = "https://ipfs.io";

/// Authentication method for IPFS requests.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum IpfsAuthentication {
    /// No authentication required.
    #[default]
    None,
    /// Use a custom header for authentication.
    Header {
        /// The header key (e.g., "Authorization").
        key: String,
        /// The header value (e.g., "Bearer ...").
        value: String,
    },
}

/// Configuration for the IPFS registry source.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpfsConfig {
    /// The IPFS endpoint URL.
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    /// The authentication method for IPFS requests.
    pub authentication: IpfsAuthentication,
    /// Optional timeout for IPFS requests.
    pub timeout: Option<Duration>,
}

/// Returns the default IPFS endpoint URL.
fn default_endpoint() -> String {
    DEFAULT_IPFS_ENDPOINT.to_string()
}

impl Default for IpfsConfig {
    /// Creates a new `IpfsConfig` with default values.
    fn default() -> Self {
        IpfsConfig {
            endpoint: default_endpoint(),
            authentication: IpfsAuthentication::default(),
            timeout: None,
        }
    }
}
