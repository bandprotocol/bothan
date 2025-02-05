use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuildError;
use crate::api::types::{API_KEY_HEADER, DEFAULT_PRO_URL, DEFAULT_URL, DEFAULT_USER_AGENT};
use crate::api::RestApi;

/// Builder for creating instances of `CoinGeckoRestAPI`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](RestApiBuilder::build) method.
///
/// # Example
/// ```
/// use bothan_coingecko::api::RestApiBuilder;
///
///
/// #[tokio::main]
/// async fn main() {
///     let mut api = RestApiBuilder::default()
///         .with_url("https://api.coingecko.com/api/v3")
///         .with_api_key("your_api_key")
///         .with_user_agent("your_user_agent")
///         .build()
///         .unwrap();
///
///     // use api ...
/// }
/// ```
pub struct RestApiBuilder {
    url: String,
    user_agent: String,
    api_key: Option<String>,
    // Flag to determine if the user modified the URL.
    mod_url: bool,
}

impl RestApiBuilder {
    pub fn new<T, U, V>(url: T, user_agent: U, api_key: Option<V>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        RestApiBuilder {
            url: url.into(),
            user_agent: user_agent.into(),
            api_key: api_key.map(Into::into),
            mod_url: true,
        }
    }

    /// Sets the URL for the API.
    /// If not specified, the default URL is `DEFAULT_URL` when no API key is provided,
    /// and `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.url = url.into();
        self.mod_url = true;
        self
    }

    /// Sets the API key for the API.
    /// The default is `None`.
    pub fn with_api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the user agent for the API.
    /// The default is `DEFAULT_USER_AGENT`.
    pub fn with_user_agent<T: Into<String>>(mut self, user_agent: T) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Builds the `CoinGeckoRestAPI` instance.
    pub fn build(self) -> Result<RestApi, BuildError> {
        let mut headers = HeaderMap::new();

        let agent = HeaderValue::from_str(&self.user_agent)?;
        headers.insert("User-Agent", agent);

        let url = match (&self.mod_url, &self.api_key) {
            (true, _) => &self.url,
            (false, Some(_)) => DEFAULT_PRO_URL,
            (false, None) => DEFAULT_URL,
        };
        let parsed_url = Url::parse(url)?;

        if let Some(key) = &self.api_key {
            let mut api_key = HeaderValue::from_str(key)?;
            api_key.set_sensitive(true);
            headers.insert(API_KEY_HEADER, api_key);
        }

        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(RestApi::new(parsed_url, client))
    }
}

impl Default for RestApiBuilder {
    /// Creates a default `CoinGeckoRestAPIBuilder` instance with default values.
    fn default() -> Self {
        RestApiBuilder {
            url: DEFAULT_URL.into(),
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.into(),
            mod_url: false,
        }
    }
}
