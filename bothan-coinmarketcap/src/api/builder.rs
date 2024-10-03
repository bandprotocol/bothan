use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuildError;
use crate::api::types::DEFAULT_URL;
use crate::api::CoinMarketCapRestAPI;

/// Builds a CoinMarketCapRestAPI with custom parameters.
/// Methods can be chained to set the parameters and the
/// `CoinMarketCapRestAPI` is constructed
/// by calling the [`build`](CoinMarketCapRestAPIBuilder::build) method.
pub struct CoinMarketCapRestAPIBuilder {
    url: String,
    api_key: Option<String>,
}

impl CoinMarketCapRestAPIBuilder {
    pub fn new<T, U>(url: T, api_key: Option<U>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        CoinMarketCapRestAPIBuilder {
            url: url.into(),
            api_key: api_key.map(Into::into),
        }
    }

    /// Sets the URL for the CoinMarketCap API.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.into();
        self
    }

    /// sets the API key for the CoinMarketCap API.
    /// The API key is required to access the API.
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Creates the configured `CoinMarketCapRestAPI`.
    pub fn build(self) -> Result<CoinMarketCapRestAPI, BuildError> {
        let mut headers = HeaderMap::new();

        let parsed_url = Url::parse(&self.url)?;

        let key = match &self.api_key {
            Some(key) => key,
            None => return Err(BuildError::MissingAPIKey()),
        };

        let mut val = HeaderValue::from_str(key)?;
        val.set_sensitive(true);
        headers.insert("X-CMC_PRO_API_KEY", val);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(CoinMarketCapRestAPI::new(parsed_url, client))
    }
}

impl Default for CoinMarketCapRestAPIBuilder {
    /// Creates a new `CoinMarketCapRestAPIBuilder` with the
    /// default URL and no API key.
    fn default() -> Self {
        CoinMarketCapRestAPIBuilder {
            url: DEFAULT_URL.into(),
            api_key: None,
        }
    }
}
