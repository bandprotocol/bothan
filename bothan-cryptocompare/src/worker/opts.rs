use serde::{Deserialize, Serialize};

use crate::api::DEFAULT_URL;
use crate::worker::types::DEFAULT_CHANNEL_SIZE;

/// Options for configuring the `CryptoCompareWorkerBuilder`.
///
/// `CryptoCompareWorkerBuilderOpts` provides a way to specify custom settings for creating a `CryptoCompareWorker`.
/// This struct allows users to set optional parameters such as the WebSocket URL and the internal channel size,
/// which will be used during the construction of the `CryptoCompareWorker`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoCompareWorkerBuilderOpts {
    #[serde(default = "default_url")]
    pub url: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_internal_ch_size")]
    pub internal_ch_size: usize,
}

fn default_url() -> String {
    DEFAULT_URL.to_string()
}

fn default_internal_ch_size() -> usize {
    DEFAULT_CHANNEL_SIZE
}

impl Default for CryptoCompareWorkerBuilderOpts {
    fn default() -> Self {
        Self {
            url: default_url(),
            api_key: None,
            internal_ch_size: default_internal_ch_size(),
        }
    }
}
