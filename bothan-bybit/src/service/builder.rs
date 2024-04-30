use serde::Deserialize;
use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::BybitRestAPIBuilder;
use crate::service::BybitService;

pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Clone, Debug, Deserialize)]
pub struct BybitServiceBuilderOpts {
    pub url: Option<String>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
}

pub struct BybitServiceBuilder {
    url: String,
    update_interval: Duration,
}

impl BybitServiceBuilder {
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.into();
        self
    }

    pub fn with_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    pub fn new(opts: BybitServiceBuilderOpts) -> Self {
        Self {
            url: opts.url.unwrap_or(DEFAULT_URL.into()),
            update_interval: opts.update_interval.unwrap_or(DEFAULT_UPDATE_INTERVAL),
        }
    }

    pub async fn build(self) -> Result<BybitService, BuilderError> {
        let mut api_builder = BybitRestAPIBuilder::default();
        api_builder.with_url(&self.url);

        let api = api_builder.build()?;

        let service = BybitService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for BybitServiceBuilder {
    fn default() -> Self {
        BybitServiceBuilder {
            url: DEFAULT_URL.into(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}
