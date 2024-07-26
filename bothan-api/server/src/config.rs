use config::Config;
use serde::Deserialize;

use crate::config::grpc::GrpcConfig;
use crate::config::ipfs::IpfsConfig;
use crate::config::log::LogConfig;
use crate::config::manager::ManagerConfig;
use crate::config::store::StoreConfig;

pub mod grpc;
pub mod ipfs;
pub mod log;
pub mod manager;
pub mod store;

/// The main application configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub log: LogConfig,
    pub ipfs: IpfsConfig,
    pub store: StoreConfig,
    pub manager: ManagerConfig,
}

impl AppConfig {
    /// Creates a new `AppConfig` using the configuration file.
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;

        // Deserialize the configuration
        config.try_deserialize()
    }
}
