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
    /// Sets the URL for the API.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the API key for the API.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the update interval for the service.
    ///
    /// # Arguments
    ///
    /// * `update_interval` - The duration for the update interval.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    /// Creates a new builder instance from the provided options.
    ///
    /// # Arguments
    ///
    /// * `opts` - The options for configuring the builder.
    ///
    /// # Returns
    ///
    /// A new `CryptoCompareServiceBuilder` instance.
    pub fn new(opts: CryptoCompareServiceBuilderOpts) -> Self {
        Self {
            url: opts.url,
            api_key: Some(opts.api_key),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
        }
    }

    /// Builds the `CryptoCompareService` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `CryptoCompareService` if successful, or a `BuilderError` otherwise.
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
