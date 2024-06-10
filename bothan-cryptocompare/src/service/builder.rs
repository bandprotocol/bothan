use serde::Deserialize;
use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::CryptoCompareRestAPIBuilder;
use crate::types::DEFAULT_UPDATE_INTERVAL;
use crate::CryptoCompareService;

/// Options for configuring the `CryptoCompareServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct CryptoCompareServiceBuilderOpts {
    pub url: Option<String>,
    pub api_key: String,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
}

/// A builder for creating instances of `CryptoCompareService`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CryptoCompareServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_cryptocompare::CryptoCompareServiceBuilder;
/// use tokio::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let service = CryptoCompareServiceBuilder::default()
///         .with_url("https://min-api.cryptocompare.com/")
///         .with_api_key("your_api_key")
///         .with_update_interval(Duration::from_secs(60))
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
pub struct CryptoCompareServiceBuilder {
    url: Option<String>,
    api_key: Option<String>,
    update_interval: Duration,
}

impl CryptoCompareServiceBuilder {
    /// Creates a new builder instance from the provided options.
    pub fn new(opts: CryptoCompareServiceBuilderOpts) -> Self {
        Self {
            url: opts.url,
            api_key: Some(opts.api_key),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
        }
    }

    /// Sets the URL for the API.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the API key for the API.
    /// The default is `None`.
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the update interval for the service.
    /// The default interval is `DEFAULT_UPDATE_INTERVAL`.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    /// Builds the `CryptoCompareService` instance.
    pub async fn build(self) -> Result<CryptoCompareService, BuilderError> {
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
    /// Creates a default `CryptoCompareServiceBuilder` instance with default values.
    fn default() -> Self {
        CryptoCompareServiceBuilder {
            url: None,
            api_key: None,
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}
