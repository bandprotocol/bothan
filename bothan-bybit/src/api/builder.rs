use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::BybitRestAPI;

pub struct BybitRestAPIBuilder {
    url: String,
}

impl BybitRestAPIBuilder {
    pub fn with_url(&mut self, url: &str) -> &Self {
        self.url = url.into();
        self
    }

    pub fn build(self) -> Result<BybitRestAPI, BuilderError> {
        let parsed_url = Url::parse(&self.url)?;
        // let mut headers = HeaderMap::new();
        // headers.insert("Host", HeaderValue::from_static("value"));
        let client = ClientBuilder::new().build()?;

        Ok(BybitRestAPI::new(parsed_url, client))
    }
}

impl Default for BybitRestAPIBuilder {
    fn default() -> Self {
        BybitRestAPIBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
