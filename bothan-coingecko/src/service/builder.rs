use serde::Deserialize;
use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_USER_AGENT;
use crate::api::CoinGeckoRestAPIBuilder;
use crate::types::{
    DEFAULT_PAGE_SIZE, DEFAULT_UPDATE_INTERVAL, DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL,
};
use crate::CoinGeckoService;

/// Options for configuring the `CoinGeckoServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
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

/// A builder for creating instances of `CoinGeckoService`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinGeckoServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_coingecko::CoinGeckoServiceBuilder;
/// use tokio::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let service = CoinGeckoServiceBuilder::default()
///         .with_url("https://api.coingecko.com/api/v3")
///         .with_api_key("your_api_key")
///         .with_user_agent("your_user_agent")
///         .with_update_interval(Duration::from_secs(60))
///         .with_page_size(100)
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
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
    /// Sets the URL for the API.
    /// The default URL is `DEFAULT_URL` when no API key is provided, and `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the API key.
    /// The default is `None`.
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the User-Agent header.
    /// The default is `DEFAULT_USER_AGENT`.
    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Sets the update interval for the service.
    /// The default is `DEFAULT_UPDATE_INTERVAL`.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    /// Sets the update interval for supported assets.
    /// The default is `DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL`.
    pub fn with_update_supported_assets_interval(
        mut self,
        update_supported_assets_interval: Duration,
    ) -> Self {
        self.update_supported_assets_interval = update_supported_assets_interval;
        self
    }

    /// Sets the page size for the service.
    /// The default is `DEFAULT_PAGE_SIZE`.
    pub fn with_page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }

    /// Sets the page query delay for the service.
    /// The default is `None`.
    pub fn with_page_query_delay(mut self, page_query_delay: Duration) -> Self {
        self.page_query_delay = Some(page_query_delay);
        self
    }

    /// Creates a new builder instance from the provided options.
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

    /// Builds the `CoinGeckoService` instance.
    pub async fn build(self) -> Result<CoinGeckoService, BuilderError> {
        let mut api_builder = CoinGeckoRestAPIBuilder::default();
        if let Some(url) = &self.url {
            api_builder.with_url(url);
        };
        if let Some(api_key) = &self.api_key {
            api_builder.with_api_key(api_key);
        };
        api_builder.with_user_agent(&self.user_agent);
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
    /// Creates a default `CoinGeckoServiceBuilder` instance with default values.
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
