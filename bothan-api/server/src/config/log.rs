//! Bothan API server logging configuration.
//!
//! Controls log levels for application, core, and sources.
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

/// Logging levels for Bothan API server.
///
/// Controls the verbosity of logging output. Higher levels include all messages from lower levels.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Most detailed logging for debugging
    Trace,
    /// Detailed information for development
    Debug,
    /// General information about application flow
    Info,
    /// Warning messages for potential issues
    Warn,
    /// Error messages for actual problems
    Error,
}

impl Display for LogLevel {
    /// Formats the log level as a lowercase string.
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

/// Logging configuration for Bothan API server.
///
/// Allows fine-grained control over log output for the main application, core library, and data sources.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    /// Log level for the main application (default: Info)
    #[serde(default = "info")]
    pub log_level: LogLevel,
    /// Log level for the core library (default: Error)
    #[serde(default = "error")]
    pub core_log_level: LogLevel,
    /// Log level for data source integrations (default: Error)
    #[serde(default = "error")]
    pub source_log_level: LogLevel,
}

// Returns the default info log level.
fn info() -> LogLevel {
    LogLevel::Info
}

// Returns the default error log level.
fn error() -> LogLevel {
    LogLevel::Error
}

impl Default for LogConfig {
    /// Creates a new `LogConfig` with default values.
    fn default() -> Self {
        LogConfig {
            log_level: info(),
            core_log_level: error(),
            source_log_level: error(),
        }
    }
}
