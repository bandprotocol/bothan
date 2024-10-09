use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Display for LogLevel {
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

/// The configuration for bothan-api's logging.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    /// The log level to use for the main application.
    #[serde(default = "info")]
    pub log_level: LogLevel,
    /// The log level to use for the core lib
    #[serde(default = "error")]
    pub core_log_level: LogLevel,
    /// The log level to use for the sources
    #[serde(default = "error")]
    pub source_log_level: LogLevel,
}

fn info() -> LogLevel {
    LogLevel::Info
}

fn error() -> LogLevel {
    LogLevel::Error
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            log_level: info(),
            core_log_level: error(),
            source_log_level: error(),
        }
    }
}
