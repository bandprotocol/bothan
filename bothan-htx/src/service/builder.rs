use serde::Deserialize;
use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::HtxRestAPIBuilder;
use crate::service::HtxService;

/// The default update interval for the service.
pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/// Options for configuring the `HtxServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct HtxServiceBuilderOpts {
    pub url: Option<String>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
}

/// A builder for creating instances of `HtxService`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](HtxServiceBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_htx::HtxServiceBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let service = HtxServiceBuilder::default()
///         .with_url("https://api.huobi.pro/")
///         .with_update_interval(Duration::from_secs(60))
///         .build()
///         .await
///         .unwrap();
///
///     // use service ...
/// }
/// ```
pub struct HtxServiceBuilder {
    url: String,
    update_interval: Duration,
}

impl HtxServiceBuilder {
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
        self.url = url.into();
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
    /// A new `HtxServiceBuilder` instance.
    pub fn new(opts: HtxServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.into()),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
        }
    }

    /// Builds the `HtxService` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `HtxService` if successful, or a `BuilderError` otherwise.
    pub async fn build(self) -> Result<HtxService, BuilderError> {
        let mut api_builder = HtxRestAPIBuilder::default();
        api_builder.with_url(&self.url);

        let api = api_builder.build()?;

        let service = HtxService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for HtxServiceBuilder {
    /// Creates a default `HtxServiceBuilder` instance with default values.
    fn default() -> Self {
        HtxServiceBuilder {
            url: DEFAULT_URL.into(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}
