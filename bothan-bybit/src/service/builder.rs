use serde::Deserialize;
use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::BybitRestAPIBuilder;
use crate::service::BybitService;

/// The default interval for updating the service.
pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

/// Options for configuring the `BybitServiceBuilder`.
#[derive(Clone, Debug, Deserialize)]
pub struct BybitServiceBuilderOpts {
    /// The URL for the Bybit API.
    pub url: Option<String>,
    /// The interval for updating the service.
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
}

/// Builder for creating instances of `BybitService`.
pub struct BybitServiceBuilder {
    url: String,
    update_interval: Duration,
}

impl BybitServiceBuilder {
    /// Sets the URL for the service.
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
    /// A new `BybitServiceBuilder` instance.
    pub fn new(opts: BybitServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.into()),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
        }
    }

    /// Builds the `BybitService` instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `BybitService` if successful, or a `BuilderError` otherwise.
    pub async fn build(self) -> Result<BybitService, BuilderError> {
        let mut api_builder = BybitRestAPIBuilder::default();
        api_builder.with_url(&self.url);

        let api = api_builder.build()?;

        let service = BybitService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for BybitServiceBuilder {
    /// Creates a default `BybitServiceBuilder` instance with the default URL and update interval.
    fn default() -> Self {
        BybitServiceBuilder {
            url: DEFAULT_URL.into(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}
