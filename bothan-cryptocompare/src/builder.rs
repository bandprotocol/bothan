use serde::Deserialize;
use tokio::time::Duration;

use crate::api::CryptoCompareRestAPIBuilder;
use crate::error::Error;
use crate::types::DEFAULT_UPDATE_INTERVAL;
use crate::CryptoCompareService;

#[derive(Debug, Deserialize)]
pub struct CryptoCompareServiceBuilderOpts {
    pub url: Option<String>,
    pub api_key: String,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
}

pub struct CryptoCompareServiceBuilder {
    url: Option<String>,
    api_key: Option<String>,
    update_interval: Duration,
}

impl CryptoCompareServiceBuilder {
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    pub fn new(opts: CryptoCompareServiceBuilderOpts) -> Self {
        Self {
            url: opts.url,
            api_key: Some(opts.api_key),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
        }
    }

    pub async fn build(self) -> Result<CryptoCompareService, Error> {
        let mut api_builder = CryptoCompareRestAPIBuilder::default();
        if let Some(url) = &self.url {
            api_builder.set_url(url);
        };
        if let Some(api_key) = &self.api_key {
            api_builder.set_api_key(api_key);
        };

        let api = api_builder.build()?;
        let service = CryptoCompareService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for CryptoCompareServiceBuilder {
    fn default() -> Self {
        CryptoCompareServiceBuilder {
            url: None,
            api_key: None,
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}
