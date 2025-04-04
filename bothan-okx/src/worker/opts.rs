use serde::{Deserialize, Serialize};

use crate::api::types::DEFAULT_URL;

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
        Self {
            url: default_url(),
        }
    }
}
