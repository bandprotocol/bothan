use std::time::Duration;

use serde::Deserialize;

use crate::api::types::DEFAULT_USER_AGENT;
use crate::worker::types::{DEFAULT_PAGE_QUERY_DELAY, DEFAULT_PAGE_SIZE, DEFAULT_UPDATE_INTERVAL};

/// Options for configuring the `CoinGeckoWorkerBuilder`.
///
/// `CoinGeckoWorkerBuilderOpts` provides a way to specify custom settings for creating a `CoinGeckoWorker`.
/// This struct allows users to set optional parameters such as the WebSocket URL and the internal channel size,
/// which will be used during the construction of the `CoinGeckoWorker`.
#[derive(Clone, Debug, Deserialize)]
pub struct CoinGeckoWorkerBuilderOpts {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    #[serde(default = "default_page_query_delay")]
    #[serde(with = "humantime_serde")]
    pub page_query_delay: Duration,
}

fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_string()
}

fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

fn default_page_size() -> usize {
    DEFAULT_PAGE_SIZE
}

fn default_page_query_delay() -> Duration {
    DEFAULT_PAGE_QUERY_DELAY
}

impl Default for CoinGeckoWorkerBuilderOpts {
    fn default() -> Self {
        Self {
            url: None,
            api_key: None,
            user_agent: default_user_agent(),
            update_interval: default_update_interval(),
            page_size: default_page_size(),
            page_query_delay: default_page_query_delay(),
        }
    }
}
