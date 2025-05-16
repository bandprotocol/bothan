use serde::{Deserialize, Serialize};

use crate::api::types::DEFAULT_URL;
use crate::worker::MAX_SUBSCRIPTION_PER_CONNECTION;

/// Options for configuring the `CoinbaseWorkerBuilder`.
///
/// `CoinbaseWorkerBuilderOpts` provides a way to specify custom settings for creating a
/// `CoinbaseWorker`. This struct allows users to set optional parameters such as the WebSocket URL
/// and the internal channel size, which will be used during the construction of the
/// `CoinbaseWorker`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    #[serde(default = "default_url")]
    pub url: String,

    #[serde(default = "default_max_subscription_per_connection")]
    pub max_subscription_per_connection: usize,
}

fn default_url() -> String {
    DEFAULT_URL.to_string()
}

fn default_max_subscription_per_connection() -> usize {
    MAX_SUBSCRIPTION_PER_CONNECTION
}

impl Default for WorkerOpts {
    fn default() -> Self {
        Self {
            url: default_url(),
            max_subscription_per_connection: default_max_subscription_per_connection(),
        }
    }
}
