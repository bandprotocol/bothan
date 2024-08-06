use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

/// The configuration for bothan-api's gRPC server.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrpcConfig {
    /// The address to bind the gRPC server too.
    #[serde(default = "default_addr")]
    pub addr: SocketAddr,
}

fn default_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 50051))
}

impl Default for GrpcConfig {
    fn default() -> Self {
        GrpcConfig {
            addr: default_addr(),
        }
    }
}
