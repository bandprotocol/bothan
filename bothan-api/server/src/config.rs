//! Bothan API server configuration module.
//!
//! Centralizes configuration management for all server components.
//!
//! ## Configuration Components
//!
//! - **gRPC**: Server endpoint and protocol configuration
//! - **IPFS**: InterPlanetary File System integration settings
//! - **Log**: Logging configuration and output settings
//! - **Manager**: Data manager and processing configuration
//! - **Monitoring**: Health monitoring and metrics settings
//! - **Store**: Data storage and persistence configuration
//! - **Telemetry**: Observability and tracing configuration
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::AppConfig;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load from a named configuration file
//!     let config = AppConfig::with_name("config")?;
//!     
//!     // Or load from a specific path
//!     let config = AppConfig::from("path/to/config.toml")?;
//!     
//!     println!("gRPC port: {}", config.grpc.addr);
//!     Ok(())
//! }
//! ```

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

/// The main application configuration for the Bothan API Server.
///
/// This struct contains all configuration settings for the server,
/// including gRPC endpoints, logging, storage, monitoring, and more.
/// All fields are optional with sensible defaults.
///
/// ## Example Configuration
///
/// ```toml
/// [grpc]
/// port = 9090
/// host = "0.0.0.0"
///
/// [log]
/// level = "info"
/// format = "json"
///
/// [store]
/// path = "/data/bothan"
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    /// gRPC server configuration
    pub grpc: GrpcConfig,
    /// Logging configuration
    pub log: LogConfig,
    /// IPFS integration configuration
    pub ipfs: IpfsConfig,
    /// Data storage configuration
    pub store: StoreConfig,
    /// Health monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Data manager configuration
    pub manager: ManagerConfig,
    /// Telemetry and observability configuration
    pub telemetry: TelemetryConfig,
}

impl AppConfig {
    /// Creates a new `AppConfig` by loading from a configuration file.
    ///
    /// The configuration file should be in TOML format and contain
    /// sections for each configuration component.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the configuration file (without extension)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the loaded configuration or a `ConfigError`
    /// if the file cannot be loaded or parsed.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bothan_api::config::AppConfig;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = AppConfig::with_name("config")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn with_name<N: AsRef<str>>(name: N) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(name.as_ref()))
            .build()?;

        // Deserialize the configuration
        config.try_deserialize()
    }

    /// Creates a new `AppConfig` by loading from a specific file path.
    ///
    /// This method allows loading configuration from any file path,
    /// not just the standard configuration file locations.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the loaded configuration or a `ConfigError`
    /// if the file cannot be loaded or parsed.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bothan_api::config::AppConfig;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = AppConfig::from("custom/path/config.toml")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .build()?;

        // Deserialize the configuration
        config.try_deserialize()
    }
}
