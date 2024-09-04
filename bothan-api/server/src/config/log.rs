use serde::{Deserialize, Serialize};

/// The configuration for bothan-api's logging.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    /// The log level to use for the application.
    #[serde(default = "default_level")]
    pub level: String,
}

fn default_level() -> String {
    "info".to_string()
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            level: default_level(),
        }
    }
}
