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
/// use tokio::time::Duration;
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
    /// Creates a new builder instance from the provided options.
    pub fn new(opts: HtxServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.into()),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
        }
    }

    /// Sets the URL for the API.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.into();
        self
    }

    /// Sets the update interval for the service.
    /// The default update interval is `DEFAULT_UPDATE_INTERVAL`.
    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    /// Builds the `HtxService` instance.
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
