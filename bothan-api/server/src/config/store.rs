use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreConfig {
    #[serde(default = "default_path", deserialize_with = "deserialize_path")]
    pub path: PathBuf,
}

fn default_path() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan")
}

fn deserialize_path<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}

impl Default for StoreConfig {
    fn default() -> Self {
        StoreConfig {
            path: default_path(),
        }
    }
}
