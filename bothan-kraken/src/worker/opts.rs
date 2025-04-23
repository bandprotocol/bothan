use serde::{Deserialize, Serialize};

use crate::api::types::DEFAULT_URL;

/// Options for configuring the `KrakenWorkerBuilder`.
///
/// `KrakenWorkerBuilderOpts` provides a way to specify custom settings for creating a
/// `KrakenWorker`. This struct allows users to set optional parameters such as the WebSocket URL
/// and the internal channel size, which will be used during the construction of the `KrakenWorker`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    #[serde(default = "default_url")]
    pub url: String,
}

fn default_url() -> String {
    DEFAULT_URL.to_string()
}

impl Default for WorkerOpts {
    fn default() -> Self {
        Self { url: default_url() }
    }
}
