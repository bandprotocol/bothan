//! Bothan API server gRPC configuration.
//!
//! Defines network settings for the gRPC server.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::grpc::GrpcConfig;
//! use std::net::SocketAddr;
//!
//! // Use default configuration
//! let config = GrpcConfig::default();
//!
//! // Custom configuration
//! let config = GrpcConfig {
//!     addr: SocketAddr::from(([127, 0, 0, 1], 9090)),
//! };
//! ```
//!
//! ## Configuration Example
//!
//! ```toml
//! [grpc]
//! addr = "0.0.0.0:50051"
//! ```

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

/// Network configuration for the Bothan API Server's gRPC server.
///
/// Specifies the address and port to bind the gRPC server to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrpcConfig {
    /// The network address and port to bind the gRPC server to (default: 0.0.0.0:50051).
    #[serde(default = "default_addr")]
    pub addr: SocketAddr,
}

fn default_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 50051))
}

impl Default for GrpcConfig {
    /// Creates a new `GrpcConfig` with default values.
    fn default() -> Self {
        GrpcConfig {
            addr: default_addr(),
        }
    }
}
