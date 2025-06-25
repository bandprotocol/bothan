//! # Logging Configuration Module
//!
//! This module provides configuration settings for the logging system
//! used throughout the Bothan API Server.
//!
//! ## Overview
//!
//! The logging configuration handles:
//! - Log level settings for different components
//! - Hierarchical logging control
//! - Output formatting and filtering
//!
//! ## Log Levels
//!
//! The system supports five log levels in order of increasing severity:
//! - **Trace**: Most detailed logging for debugging
//! - **Debug**: Detailed information for development
//! - **Info**: General information about application flow
//! - **Warn**: Warning messages for potential issues
//! - **Error**: Error messages for actual problems
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::log::{LogConfig, LogLevel};
//!
//! // Use default configuration
//! let config = LogConfig::default();
//!
//! // Custom configuration
//! let config = LogConfig {
//!     log_level: LogLevel::Debug,
//!     core_log_level: LogLevel::Info,
//!     source_log_level: LogLevel::Warn,
//! };
//! ```
//!
//! ## Configuration Example
//!
//! ```toml
//! [log]
//! log_level = "info"
//! core_log_level = "error"
//! source_log_level = "error"
//! ```

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents the different logging levels available in the system.
///
/// Log levels are used to control the verbosity of logging output.
/// Higher levels include all messages from lower levels.
///
/// ## Level Hierarchy
///
/// The log levels follow this order of increasing severity:
/// - Trace (lowest)
/// - Debug
/// - Info
/// - Warn
/// - Error (highest)
///
/// ## Serialization
///
/// When serialized to TOML or JSON, log levels are converted to lowercase strings.
///
/// # Examples
///
/// ```rust,no_run
/// use bothan_api::config::log::LogLevel;
///
/// let level = LogLevel::Info;
/// println!("{}", level); // Output: "info"
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Trace level - most detailed logging for debugging
    Trace,
    /// Debug level - detailed information for development
    Debug,
    /// Info level - general information about application flow
    Info,
    /// Warn level - warning messages for potential issues
    Warn,
    /// Error level - error messages for actual problems
    Error,
}

impl Display for LogLevel {
    /// Formats the log level as a lowercase string.
    ///
    /// This implementation is used when converting log levels to strings
    /// for configuration files and logging output.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    ///
    /// Returns a `std::fmt::Result` indicating success or failure.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bothan_api::config::log::LogLevel;
    ///
    /// let level = LogLevel::Info;
    /// assert_eq!(level.to_string(), "info");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            LogLevel::Trace => "trace".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Error => "error".to_string(),
        };
        write!(f, "{}", str)
    }
}

/// Configuration for the Bothan API Server's logging system.
///
/// This struct defines logging levels for different components of the system,
/// allowing fine-grained control over log output verbosity.
///
/// ## Component-Specific Logging
///
/// The system supports different log levels for different components:
/// - **Main Application**: General application logging
/// - **Core Library**: Bothan core library operations
/// - **Data Sources**: Exchange and data source integrations
///
/// ## Default Configuration
///
/// By default, the main application uses Info level, while core and source
/// components use Error level to reduce noise.
///
/// ## Example Configuration
///
/// ```toml
/// [log]
/// # Main application logging
/// log_level = "info"
///
/// # Core library logging (more verbose for debugging)
/// core_log_level = "debug"
///
/// # Data source logging (minimal to reduce noise)
/// source_log_level = "warn"
/// ```
///
/// ## Usage Patterns
///
/// - **Development**: Set all levels to `debug` or `trace` for maximum detail
/// - **Production**: Use `info` for main app, `error` for core/sources
/// - **Troubleshooting**: Increase specific component levels as needed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    /// The log level to use for the main application.
    ///
    /// This controls logging for the primary server components,
    /// including HTTP/gRPC handlers, configuration loading, and
    /// general application flow.
    ///
    /// Default: `LogLevel::Info`
    #[serde(default = "info")]
    pub log_level: LogLevel,

    /// The log level to use for the core library components.
    ///
    /// This controls logging for the Bothan core library operations,
    /// including data processing, storage operations, and internal
    /// system management.
    ///
    /// Default: `LogLevel::Error`
    #[serde(default = "error")]
    pub core_log_level: LogLevel,

    /// The log level to use for data source integrations.
    ///
    /// This controls logging for exchange integrations and data sources,
    /// including API calls, WebSocket connections, and data parsing.
    /// Set to higher levels to reduce noise from external services.
    ///
    /// Default: `LogLevel::Error`
    #[serde(default = "error")]
    pub source_log_level: LogLevel,
}

/// Returns the default info log level.
///
/// This function is used as a default value for the main application
/// log level in configuration deserialization.
///
/// # Returns
///
/// Returns `LogLevel::Info` as the default logging level.
///
/// # Example
///
/// ```rust,no_run
/// use bothan_api::config::log::LogLevel;
///
/// let level = LogLevel::Info;
/// assert_eq!(level, LogLevel::Info);
/// ```
pub fn info() -> LogLevel {
    LogLevel::Info
}

/// Returns the default error log level.
///
/// This function is used as a default value for core and source
/// log levels in configuration deserialization.
///
/// # Returns
///
/// Returns `LogLevel::Error` as the default logging level.
///
/// # Example
///
/// ```rust,no_run
/// use bothan_api::config::log::LogLevel;
///
/// let level = LogLevel::Error;
/// assert_eq!(level, LogLevel::Error);
/// ```
pub fn error() -> LogLevel {
    LogLevel::Error
}

impl Default for LogConfig {
    /// Creates a new `LogConfig` with default values.
    ///
    /// The default configuration uses:
    /// - Info level for main application logging
    /// - Error level for core library logging
    /// - Error level for data source logging
    ///
    /// # Returns
    ///
    /// Returns a `LogConfig` instance with sensible default logging levels.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bothan_api::config::log::{LogConfig, LogLevel};
    ///
    /// let config = LogConfig::default();
    /// assert_eq!(config.log_level, LogLevel::Info);
    /// assert_eq!(config.core_log_level, LogLevel::Error);
    /// assert_eq!(config.source_log_level, LogLevel::Error);
    /// ```
    fn default() -> Self {
        LogConfig {
            log_level: info(),
            core_log_level: error(),
            source_log_level: error(),
        }
    }
}
