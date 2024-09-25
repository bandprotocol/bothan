use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::api::types::DEFAULT_URL;
use crate::worker::types::{DEFAULT_UPDATE_INTERVAL, DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL};

/// Options for configuring the `CoinMarketCapWorkerBuilder`.
///
/// `CoinMarketCapWorkerBuilderOpts` provides a way to specify custom settings for creating a `CoinMarketCapWorker`.
/// This struct allows users to set optional parameters such as the WebSocket URL and the internal channel size,
/// which will be used during the construction of the `CoinMarketCapWorker`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoinMarketCapWorkerBuilderOpts {
    #[serde(default = "default_url")]
    pub url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
    #[serde(default = "default_update_supported_assets_interval")]
    #[serde(with = "humantime_serde")]
    pub update_supported_assets_interval: Duration,
}

fn default_url() -> String {
    DEFAULT_URL.to_string()
}

fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

fn default_update_supported_assets_interval() -> Duration {
    DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL
}

impl Default for CoinMarketCapWorkerBuilderOpts {
    fn default() -> Self {
        Self {
            url: default_url(),
            api_key: None,
            update_interval: default_update_interval(),
            update_supported_assets_interval: default_update_supported_assets_interval(),
        }
    }
}
