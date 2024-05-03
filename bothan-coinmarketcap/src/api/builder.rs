use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError as Error;
use crate::api::types::DEFAULT_URL;
use crate::api::CoinMarketCapRestAPI;

/// Builds a CoinMarketCapRestAPI with custom parameters.
/// Methods can be chained to set the parameters and the
/// [`CoinMarketCapRestAPI`](CoinMarketCapRestAPI) is constructed
/// by calling the [`build`](CoinMarketCapRestAPIBuilder::build) method.
/// # Example
/// ```no_run rust
/// use bothan_coinmarketcap::api::builder::CoinMarketCapRestAPIBuilder;
///
/// async fn main() {
///     let api = CoinMarketCapRestAPIBuilder::default()
///         .with_api_key("your_api_key")
///         .build()
///         .unwrap();
///
///     // use the api ...
/// }
/// ```
pub struct CoinMarketCapRestAPIBuilder {
    url: String,
    api_key: Option<String>,
}

impl CoinMarketCapRestAPIBuilder {
    /// Sets the URL for the CoinMarketCap API.
    /// The default URL is [`DEFAULT_URL`](DEFAULT_URL).
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

    /// Creates the configured [`CoinMarketCapRestAPI`](CoinMarketCapRestAPI).
    pub fn build(self) -> Result<CoinMarketCapRestAPI, Error> {
        let mut headers = HeaderMap::new();

        let parsed_url = Url::parse(&self.url)?;

        let key = match &self.api_key {
            Some(key) => key,
            None => return Err(Error::MissingAPIKey()),
        };

        let mut val = HeaderValue::from_str(key)?;
        val.set_sensitive(true);
        headers.insert("X-CMC_PRO_API_KEY", val);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(CoinMarketCapRestAPI::new(parsed_url, client))
    }
}

impl Default for CoinMarketCapRestAPIBuilder {
    /// Creates a new [`CoinMarketCapRestAPIBuilder`](CoinMarketCapRestAPIBuilder) with the
    /// default URL and no API key.
    fn default() -> Self {
        CoinMarketCapRestAPIBuilder {
            url: DEFAULT_URL.into(),
            api_key: None,
        }
    }
}
