use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api::types::DEFAULT_URL;
use crate::worker::types::DEFAULT_UPDATE_INTERVAL;

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
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(serialize_with = "none_is_empty_string")]
    pub api_key: Option<String>,
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

impl Default for CoinMarketCapWorkerBuilderOpts {
    fn default() -> Self {
        Self {
            url: default_url(),
            api_key: None,
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
