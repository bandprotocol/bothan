use tokio::time::Duration;

use crate::api::CoinGeckoRestAPIBuilder;
use crate::error::Error;
use crate::types::{DEFAULT_UPDATE_INTERVAL, DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL};
use crate::CoinGeckoService;

#[derive(Default)]
pub struct CoinGeckoServiceBuilder {
    url: Option<String>,
    api_key: Option<String>,
    user_agent: Option<String>,
    update_interval: Option<Duration>,
    update_supported_assets_interval: Option<Duration>,
}

impl CoinGeckoServiceBuilder {
    pub fn set_url(mut self, url: &str) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn set_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn set_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn set_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = Some(update_interval);
        self
    }

    pub fn set_update_supported_assets_interval(
        mut self,
        update_supported_assets_interval: Duration,
    ) -> Self {
        self.update_supported_assets_interval = Some(update_supported_assets_interval);
        self
    }

    pub async fn build(self) -> Result<CoinGeckoService, Error> {
        let mut api_builder = CoinGeckoRestAPIBuilder::default();
        if let Some(url) = &self.url {
            api_builder.set_url(url);
        };

        if let Some(api_key) = &self.api_key {
            api_builder.set_api_key(api_key);
        };

        if let Some(user_agent) = &self.user_agent {
            api_builder.set_user_agent(user_agent);
        };

        let update_interval = self.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL);
        let update_supported_assets_interval = self
            .update_supported_assets_interval
            .unwrap_or(DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL);

        let api = api_builder.build()?;
        Ok(CoinGeckoService::new(api, update_interval, update_supported_assets_interval).await)
    }
}
