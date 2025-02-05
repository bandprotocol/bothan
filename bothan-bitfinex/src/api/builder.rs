use crate::api::error::BuildError;
use crate::api::rest::{RestApi, DEFAULT_URL};
use reqwest::ClientBuilder;
use url::Url;

pub struct RestApiBuilder {
    url: String,
}

impl RestApiBuilder {
    pub fn new<T: Into<String>>(url: T) -> Self {
        RestApiBuilder { url: url.into() }
    }

    /// Sets the URL for the API.
    /// If not specified, the default URL is `DEFAULT_URL` when no API key is provided,
    /// and `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.url = url.into();
        self
    }

    /// Builds the `CoinGeckoRestAPI` instance.
    pub fn build(self) -> Result<RestApi, BuildError> {
        let parsed_url = Url::parse(&self.url)?;

        let client = ClientBuilder::new().build()?;

        Ok(RestApi::new(parsed_url, client))
    }
}

impl Default for RestApiBuilder {
    /// Creates a default `CoinGeckoRestAPIBuilder` instance with default values.
    fn default() -> Self {
        RestApiBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
