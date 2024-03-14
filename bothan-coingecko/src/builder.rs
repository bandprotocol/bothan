use serde::Deserialize;
use tokio::time::Duration;

use crate::api::types::DEFAULT_USER_AGENT;
use crate::api::CoinGeckoRestAPIBuilder;
use crate::error::Error;
use crate::types::{
    DEFAULT_PAGE_SIZE, DEFAULT_UPDATE_INTERVAL, DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL,
};
use crate::CoinGeckoService;

#[derive(Debug, Deserialize)]
pub struct CoinGeckoServiceBuilderOpts {
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

    pub fn new(opts: CoinGeckoServiceBuilderOpts) -> Self {
        Self {
            url: opts.url,
            api_key: opts.api_key,
            user_agent: opts.user_agent.unwrap_or(DEFAULT_USER_AGENT.into()),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
            update_supported_assets_interval: opts
                .update_supported_assets_interval
                .unwrap_or(DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL),
            page_size: opts.page_size.unwrap_or(DEFAULT_PAGE_SIZE),
            page_query_delay: opts.page_query_delay,
        }
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
