use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::HtxRestAPI;

pub struct HtxRestAPIBuilder {
    url: String,
}

impl HtxRestAPIBuilder {
    pub fn set_url(&mut self, url: &str) -> &Self {
        self.url = url.into();
        self
    }

    pub fn build(self) -> Result<HtxRestAPI, BuilderError> {
        let parsed_url = Url::parse(&self.url)?;

        let client = ClientBuilder::new().build()?;

        Ok(HtxRestAPI::new(parsed_url, client))
    }
}

impl Default for HtxRestAPIBuilder {
    fn default() -> Self {
        HtxRestAPIBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
