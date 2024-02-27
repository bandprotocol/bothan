use tokio::time::Duration;

use crate::api::types::DEFAULT_USER_AGENT;
use crate::api::CoinGeckoRestAPIBuilder;
use crate::error::Error;
use crate::types::{
    DEFAULT_PAGE_SIZE, DEFAULT_UPDATE_INTERVAL, DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL,
};
use crate::CoinGeckoService;

pub struct CoinGeckoServiceBuilder {
    url: Option<String>,
    api_key: Option<String>,
    user_agent: String,
    update_interval: Duration,
    update_supported_assets_interval: Duration,
    page_size: usize,
    page_query_delay: Option<Duration>,
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
        self.user_agent = user_agent.into();
        self
    }

    pub fn set_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    pub fn set_update_supported_assets_interval(
        mut self,
        update_supported_assets_interval: Duration,
    ) -> Self {
        self.update_supported_assets_interval = update_supported_assets_interval;
        self
    }

    pub fn set_page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn set_page_query_delay(mut self, page_query_delay: Duration) -> Self {
        self.page_query_delay = Some(page_query_delay);
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
        api_builder.set_user_agent(&self.user_agent);
        let api = api_builder.build()?;

        let service = CoinGeckoService::new(
            api,
            self.update_interval,
            self.update_supported_assets_interval,
            self.page_size,
            self.page_query_delay,
        )
        .await;

        Ok(service)
    }
}

impl Default for CoinGeckoServiceBuilder {
    fn default() -> Self {
        CoinGeckoServiceBuilder {
            url: None,
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.into(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
            update_supported_assets_interval: DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL,
            page_size: DEFAULT_PAGE_SIZE,
            page_query_delay: None,
        }
    }
}
