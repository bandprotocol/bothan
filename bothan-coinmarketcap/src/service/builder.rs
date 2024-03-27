use serde::Deserialize;
use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::CoinMarketCapRestAPIBuilder;
use crate::CoinMarketCapService;

pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL: Duration = Duration::from_secs(86400);

#[derive(Debug, Deserialize)]
pub struct CoinMarketCapServiceBuilderOpts {
    pub url: Option<String>,
    pub api_key: String,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_supported_assets_interval: Option<Duration>,
}

pub struct CoinMarketCapServiceBuilder {
    url: Option<String>,
    api_key: Option<String>,
    update_interval: Duration,
    update_supported_assets_interval: Duration,
}

impl CoinMarketCapServiceBuilder {
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

    pub fn with_update_supported_assets_interval(
        mut self,
        update_supported_assets_interval: Duration,
    ) -> Self {
        self.update_supported_assets_interval = update_supported_assets_interval;
        self
    }

    pub fn new(opts: CoinMarketCapServiceBuilderOpts) -> Self {
        Self {
            url: opts.url,
            api_key: Some(opts.api_key),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
            update_supported_assets_interval: opts
                .update_supported_assets_interval
                .unwrap_or(DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL),
        }
    }

    pub async fn build(self) -> Result<CoinMarketCapService, BuilderError> {
        let mut api_builder = CoinMarketCapRestAPIBuilder::default();
        if let Some(url) = &self.url {
            api_builder.with_url(url);
        };
        if let Some(api_key) = &self.api_key {
            api_builder.with_api_key(api_key);
        };
        let api = api_builder.build()?;

        let service = CoinMarketCapService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for CoinMarketCapServiceBuilder {
    fn default() -> Self {
        CoinMarketCapServiceBuilder {
            url: None,
            api_key: None,
            update_interval: DEFAULT_UPDATE_INTERVAL,
            update_supported_assets_interval: DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL,
        }
    }
}
