use serde::Deserialize;
use tokio::time::Duration;

#[derive(Debug, Deserialize)]
pub struct CoingeckoServiceConfig {
    pub url: Option<String>,
    pub api_key: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_supported_assets_interval: Option<Duration>,
    pub page_size: Option<usize>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub page_query_delay: Option<Duration>,
}
