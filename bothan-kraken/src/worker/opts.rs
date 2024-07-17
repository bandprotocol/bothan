use serde::Deserialize;

use crate::api::types::DEFAULT_URL;
use crate::worker::types::DEFAULT_CHANNEL_SIZE;

/// Options for configuring the `KrakenWorkerBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct KrakenWorkerBuilderOpts {
    #[serde(default = "default_url")]
    pub url: String,
    #[serde(default = "default_internal_ch_size")]
    pub internal_ch_size: usize,
}

fn default_url() -> String {
    DEFAULT_URL.to_string()
}

fn default_internal_ch_size() -> usize {
    DEFAULT_CHANNEL_SIZE
}

impl Default for KrakenWorkerBuilderOpts {
    fn default() -> Self {
        Self {
            url: default_url(),
            internal_ch_size: default_internal_ch_size(),
        }
    }
}
