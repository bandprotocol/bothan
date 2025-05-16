use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

/// The configuration for all bothan-api's telemetry.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TelemetryConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default = "default_addr")]
    pub addr: SocketAddr,
}

fn default_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 4318))
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            addr: default_addr(),
        }
    }
}
