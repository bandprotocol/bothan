//! Bothan API server store configuration.
//!
//! Settings for data storage path.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::store::StoreConfig;
//! let config = StoreConfig::default();
//! ```
//!
//! ## Configuration Example
//!
//! ```toml
//! [store]
//! path = "/home/user/.bothan/data"
//! ```

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Configuration for the Bothan API Server's data store.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreConfig {
    /// The path to the data store directory.
    #[serde(default = "default_path")]
    pub path: PathBuf,
}

/// Returns the default path for the data store directory.
fn default_path() -> PathBuf {
    let home = dirs::home_dir().expect("Failed to get home directory");
    home.join(".bothan/data")
}

impl Default for StoreConfig {
    /// Creates a new `StoreConfig` with default values.
    fn default() -> Self {
        StoreConfig {
            path: default_path(),
        }
    }
}
