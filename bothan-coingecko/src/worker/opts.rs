use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api::types::{DEFAULT_URL, DEFAULT_USER_AGENT};

const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/// Options for configuring the `CoinGeckoWorkerBuilder`.
///
/// `CoinGeckoWorkerBuilderOpts` provides a way to specify custom settings for creating a `CoinGeckoWorker`.
/// This struct allows users to set optional parameters such as the WebSocket URL and the internal channel size,
/// which will be used during the construction of the `CoinGeckoWorker`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    #[serde(serialize_with = "none_is_default_url")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(serialize_with = "none_is_empty_string")]
    pub api_key: Option<String>,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    #[serde(default = "default_update_interval")]
    #[serde(with = "humantime_serde")]
    pub update_interval: Duration,
}

fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_string()
}

fn default_update_interval() -> Duration {
    DEFAULT_UPDATE_INTERVAL
}

impl Default for WorkerOpts {
    fn default() -> Self {
        Self {
            url: None,
            api_key: None,
            user_agent: default_user_agent(),
            update_interval: default_update_interval(),
        }
    }
}

fn empty_string_is_none<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

fn none_is_empty_string<S: Serializer>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(val) => serializer.serialize_str(val),
        None => serializer.serialize_str(""),
    }
}

fn none_is_default_url<S: Serializer>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(val) => serializer.serialize_str(val),
        None => serializer.serialize_str(DEFAULT_URL),
    }
}
