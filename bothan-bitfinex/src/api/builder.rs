use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuildError;
use crate::api::rest::{DEFAULT_URL, RestApi};

pub struct RestApiBuilder {
    url: String,
}

impl RestApiBuilder {
    pub fn new<T: Into<String>>(url: T) -> Self {
        RestApiBuilder { url: url.into() }
    }

    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.url = url.into();
        self
    }

    pub fn build(self) -> Result<RestApi, BuildError> {
        let parsed_url = Url::parse(&self.url)?;

        let client = ClientBuilder::new().build()?;

        Ok(RestApi::new(parsed_url, client))
    }
}

impl Default for RestApiBuilder {
    fn default() -> Self {
        RestApiBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
