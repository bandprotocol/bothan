use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

use crate::config::grpc::GrpcConfig;
use crate::config::ipfs::IpfsConfig;
use crate::config::log::LogConfig;
use crate::config::manager::ManagerConfig;
use crate::config::monitoring::MonitoringConfig;
use crate::config::store::StoreConfig;
use crate::config::telemetry::TelemetryConfig;

pub mod grpc;
pub mod ipfs;
pub mod log;
pub mod manager;
pub mod monitoring;
pub mod store;
pub mod telemetry;

/// The main application configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub log: LogConfig,
    pub ipfs: IpfsConfig,
    pub store: StoreConfig,
    pub monitoring: MonitoringConfig,
    pub manager: ManagerConfig,
    pub telemetry: TelemetryConfig,
}

impl AppConfig {
    /// Creates a new `AppConfig` using the configuration file.
    pub fn with_name<N: AsRef<str>>(name: N) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(name.as_ref()))
            .build()?;

        // Deserialize the configuration
        config.try_deserialize()
    }

    pub fn from<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .build()?;

        // Deserialize the configuration
        config.try_deserialize()
    }
}
