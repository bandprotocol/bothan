use tokio::time::Duration;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::HtxRestAPIBuilder;
use crate::service::HtxService;

pub(crate) const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(60);

pub struct HtxServiceBuilder {
    url: String,
    update_interval: Duration,
}

impl HtxServiceBuilder {
    pub fn set_url(mut self, url: &str) -> Self {
        self.url = url.into();
        self
    }

    pub fn set_update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    pub async fn build(self) -> Result<HtxService, BuilderError> {
        let mut api_builder = HtxRestAPIBuilder::default();
        api_builder.set_url(&self.url);

        let api = api_builder.build()?;

        let service = HtxService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for HtxServiceBuilder {
    fn default() -> Self {
        HtxServiceBuilder {
            url: DEFAULT_URL.into(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}
