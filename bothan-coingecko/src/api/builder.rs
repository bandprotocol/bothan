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
///     let api = CoinGeckoRestAPIBuilder::default()
///         .with_url("https://api.coingecko.com/api/v3")
///         .with_api_key("your_api_key")
///         .with_user_agent("your_user_agent")
///         .build()
///         .unwrap();
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
    /// Sets the URL for the API.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn with_url(&mut self, url: &str) -> &Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the API key for the API.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice that holds the API key.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn with_api_key(&mut self, api_key: &str) -> &Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the user agent for the API.
    ///
    /// # Arguments
    ///
    /// * `user_agent` - A string slice that holds the user agent.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn with_user_agent(&mut self, user_agent: &str) -> &Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Builds the `CoinGeckoRestAPI` instance.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the instance was created successfully, or a `BuilderError` if there was a problem.
    pub fn build(self) -> Result<CoinGeckoRestAPI, BuilderError> {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_str(&self.user_agent)?);

        let url = match self.url {
            Some(url) => url,
            None => match &self.api_key {
                None => DEFAULT_URL.to_string(),
                Some(_) => DEFAULT_PRO_URL.to_string(),
            },
        };
        let parsed_url = Url::parse(&url)?;

        if let Some(key) = &self.api_key {
            let mut val = HeaderValue::from_str(key)?;
            val.set_sensitive(true);
            headers.insert("x-cg-pro-api-key", val);
        }

        let client = ClientBuilder::new().default_headers(headers).build()?;

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
