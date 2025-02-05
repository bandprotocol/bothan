use crate::api::rest::DEFAULT_URL;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    #[serde(default = "default_url")]
    pub url: String,

    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
}

fn default_url() -> String {
    DEFAULT_URL.to_string()
}

fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

impl Default for WorkerOpts {
    fn default() -> Self {
        Self {
            url: default_url(),
            update_interval: default_update_interval(),
        }
    }
}
