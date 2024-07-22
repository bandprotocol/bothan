use serde::Deserialize;

/// The configuration for bothan-api's logging.
#[derive(Clone, Debug, Deserialize)]
pub struct LogConfig {
    /// The log level to use for the application.
    pub level: String,
}
