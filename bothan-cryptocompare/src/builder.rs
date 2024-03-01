use tokio::time::Duration;

use crate::api::types::DEFAULT_USER_AGENT;
use crate::api::CryptoCompareRestAPIBuilder;
use crate::error::Error;
use crate::types::DEFAULT_UPDATE_INTERVAL;
use crate::CryptoCompareService;

pub struct CryptoCompareServiceBuilder {
    url: Option<String>,
    api_key: Option<String>,
    user_agent: String,
    update_interval: Duration,
}

impl CryptoCompareServiceBuilder {
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

    pub async fn build(self) -> Result<CryptoCompareService, Error> {
        let mut api_builder = CryptoCompareRestAPIBuilder::default();
        if let Some(url) = &self.url {
            api_builder.set_url(url);
        };
        if let Some(api_key) = &self.api_key {
            api_builder.set_api_key(api_key);
        };
        api_builder.set_user_agent(&self.user_agent);
        let api = api_builder.build()?;

        let service = CryptoCompareService::new(api, self.update_interval).await;

        Ok(service)
    }
}

impl Default for CryptoCompareServiceBuilder {
    fn default() -> Self {
        CryptoCompareServiceBuilder {
            url: None,
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.into(),
            update_interval: DEFAULT_UPDATE_INTERVAL,
        }
    }
}
