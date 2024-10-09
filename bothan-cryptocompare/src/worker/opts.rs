use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    #[serde(deserialize_with = "empty_string_is_none")]
    #[serde(serialize_with = "none_is_empty_string")]
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
