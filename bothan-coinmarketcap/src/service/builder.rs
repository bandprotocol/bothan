use serde::Deserialize;
use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::CoinMarketCapRestAPIBuilder;
use crate::CoinMarketCapService;

pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL: Duration = Duration::from_secs(86400);

/// Options for the `CoinMarketCapServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
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

/// Builds a CoinMarketCap service with custom options.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinMarketCapServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let service = CoinMarketCapServiceBuilder::default()
///         .with_api_key("your_api_key")
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
pub struct CoinMarketCapServiceBuilder {
    url: Option<String>,
    api_key: Option<String>,
    update_interval: Duration,
    update_supported_assets_interval: Duration,
}

impl CoinMarketCapServiceBuilder {
    /// Returns a new `CoinMarketCapServiceBuilder`.
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

    /// Sets the URL for the CoinMarketCap API.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the API key for the CoinMarketCap API.
    /// The API key is required to access the API.
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the update interval for the CoinMarketCap service.
    /// The service will update the quotes for all tracked assets at this interval.
    /// The default interval is `DEFAULT_UPDATE_INTERVAL`.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    /// Sets the update interval for the supported assets list.
    /// The service will update the list of supported assets at this interval.
    /// The default interval is `DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL`.
    pub fn with_update_supported_assets_interval(
        mut self,
        update_supported_assets_interval: Duration,
    ) -> Self {
        self.update_supported_assets_interval = update_supported_assets_interval;
        self
    }

    /// Creates the configured `CoinMarketCapService`.
    pub async fn build(self) -> Result<CoinMarketCapService, BuilderError> {
        let mut api_builder = CoinMarketCapRestAPIBuilder::default();
        if let Some(url) = &self.url {
            api_builder = api_builder.with_url(url);
        };
        if let Some(api_key) = &self.api_key {
            api_builder = api_builder.with_api_key(api_key);
        };
        let api = api_builder.build()?;

        let service = CoinMarketCapService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for CoinMarketCapServiceBuilder {
    /// Creates a new `CoinMarketCapServiceBuilder` with the default
    /// values. The API key is not set by default.
    fn default() -> Self {
        CoinMarketCapServiceBuilder {
            url: None,
            api_key: None,
            update_interval: DEFAULT_UPDATE_INTERVAL,
            update_supported_assets_interval: DEFAULT_UPDATE_SUPPORTED_ASSETS_INTERVAL,
        }
    }
}
