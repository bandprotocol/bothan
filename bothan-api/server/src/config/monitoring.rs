use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The configuration for bothan-api's monitoring service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    /// The path to where the key for the monitoring service is stored.
    #[serde(default = "default_path")]
    pub path: PathBuf,

    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_endpoint() -> String {
    "https://bothan-monitoring.bandchain.org".to_string()
}

fn default_path() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan/keyring/broadcaster.info")
}

fn default_enabled() -> bool {
    true
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        MonitoringConfig {
            endpoint: default_endpoint(),
            path: default_path(),
            enabled: default_enabled(),
        }
    }
}
