use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreConfig {
    #[serde(default = "default_path")]
    pub path: PathBuf,
}

fn default_path() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan")
}

impl Default for StoreConfig {
    fn default() -> Self {
        StoreConfig {
            path: default_path(),
        }
    }
}
