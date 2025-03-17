use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

/// The configuration for all bothan-api's telemetry.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TelemetryConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    
    #[serde(default = "default_addr")]
    pub addr: SocketAddr,
}

fn default_enabled() -> bool {
    false
}

fn default_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 4318))
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            addr: default_addr(),
        }
    }
}
