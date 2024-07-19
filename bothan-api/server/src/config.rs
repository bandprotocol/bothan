use config::Config;
use serde::Deserialize;

use crate::config::grpc::GrpcConfig;
use crate::config::ipfs::IpfsConfig;
use crate::config::log::LogConfig;
use crate::config::manager::ManagerConfig;
use crate::config::registry::RegistryConfig;
use crate::config::source::SourceConfig;

pub mod grpc;
pub mod ipfs;
pub mod log;
pub mod manager;
pub mod registry;
pub mod source;

/// The main application configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub manager: ManagerConfig,
    pub source: SourceConfig,
    pub registry: RegistryConfig,
    pub ipfs: IpfsConfig,
    pub log: LogConfig,
}

impl AppConfig {
    /// Creates a new `AppConfig` using the configuration file.
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap();

        // Deserialize the configuration
        config.try_deserialize()
    }
}
