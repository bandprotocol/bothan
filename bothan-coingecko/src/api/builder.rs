use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError;
use crate::api::types::{DEFAULT_PRO_URL, DEFAULT_URL, DEFAULT_USER_AGENT};
use crate::api::CoinGeckoRestAPI;

/// Builder for creating instances of `CoinGeckoRestAPI`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CoinGeckoRestAPIBuilder::build) method.
///
/// # Example
/// ```no_run
/// use bothan_coingecko::api::CoinGeckoRestAPIBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let mut builder = CoinGeckoRestAPIBuilder::default();
///     builder.with_url("https://api.coingecko.com/api/v3");
///     builder.with_api_key("your_api_key");
///     builder.with_user_agent("your_user_agent");
///     let api = builder.build().unwrap();
///
///     // use api ...
/// }
/// ```
pub struct CoinGeckoRestAPIBuilder {
    url: Option<String>,
    api_key: Option<String>,
    user_agent: String,
}

impl CoinGeckoRestAPIBuilder {
    pub fn new<T: Into<String>>(url: Option<T>, api_key: Option<T>, user_agent: T) -> Self {
        CoinGeckoRestAPIBuilder {
            url: url.map(|v| v.into()),
            api_key: api_key.map(|v| v.into()),
            user_agent: user_agent.into(),
        }
    }
    /// Sets the URL for the API.
    /// If not specified, the default URL is `DEFAULT_URL` when no API key is provided,
    /// and `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url<T: Into<String>>(&mut self, url: T) -> &Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the API key for the API.
    /// The default is `None`.
    pub fn with_api_key<T: Into<String>>(&mut self, api_key: T) -> &Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the user agent for the API.
    /// The default is `DEFAULT_USER_AGENT`.
    pub fn with_user_agent<T: Into<String>>(&mut self, user_agent: T) -> &Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Builds the `CoinGeckoRestAPI` instance.
    pub fn build(self) -> Result<CoinGeckoRestAPI, BuilderError> {
        let mut headers = HeaderMap::new();
        let agent = match HeaderValue::from_str(&self.user_agent) {
            Ok(agent) => agent,
            Err(_) => return Err(BuilderError::InvalidHeaderValue(self.user_agent)),
        };

        headers.insert("User-Agent", agent);

        let url = match self.url {
            Some(url) => url,
            None => match &self.api_key {
                None => DEFAULT_URL.to_string(),
                Some(_) => DEFAULT_PRO_URL.to_string(),
            },
        };
        let parsed_url = Url::parse(&url)?;

        if let Some(key) = &self.api_key {
            let mut api_key = match HeaderValue::from_str(key) {
                Ok(key) => key,
                Err(_) => return Err(BuilderError::InvalidHeaderValue(key.clone())),
            };
            api_key.set_sensitive(true);
            headers.insert("x-cg-pro-api-key", api_key);
        }

        let client = match ClientBuilder::new().default_headers(headers).build() {
            Ok(client) => client,
            Err(e) => return Err(BuilderError::BuildFailed(e.to_string())),
        };

        Ok(CoinGeckoRestAPI::new(parsed_url, client))
    }
}

impl Default for CoinGeckoRestAPIBuilder {
    /// Creates a default `CoinGeckoRestAPIBuilder` instance with default values.
    fn default() -> Self {
        CoinGeckoRestAPIBuilder {
            url: None,
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }
}
