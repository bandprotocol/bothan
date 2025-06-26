//! Bothan API server telemetry configuration.
//!
//! Settings for telemetry enable flag and server address.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::telemetry::TelemetryConfig;
//! let config = TelemetryConfig::default();
//! ```
//!
//! ## Configuration Example
//!
//! ```toml
//! [telemetry]
//! enabled = true
//! addr = "127.0.0.1:4318"
//! ```

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

/// Configuration for the Bothan API Server's telemetry system.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TelemetryConfig {
    /// Whether telemetry is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// The address to bind the telemetry server to.
    #[serde(default = "default_addr")]
    pub addr: SocketAddr,
}

/// Returns the default telemetry server address.
fn default_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 4318))
}

impl Default for TelemetryConfig {
    /// Creates a new `TelemetryConfig` with default values.
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            addr: default_addr(),
        }
    }
}
