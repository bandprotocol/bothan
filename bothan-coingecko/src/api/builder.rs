//! Builder for configuring and constructing `CoinGeckoRestAPI`.
//!
//! This module provides a builder for constructing [`RestApi`] clients used
//! to interact with the CoinGecko REST API. The builder supports optional configuration
//! of base URL, user agent, and API key.
//! 
//! The module provides:
//!
//! - The [`RestApiBuilder`] for REST API building
//! - Supports setting the API base URL, API key, and custom user agent
//! - Automatically uses the default CoinGecko URL when parameters are omitted during the [`build`](`RestApiBuilder::build`) call

use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

use crate::api::RestApi;
use crate::api::error::BuildError;
use crate::api::types::{API_KEY_HEADER, DEFAULT_PRO_URL, DEFAULT_URL, DEFAULT_USER_AGENT};

/// Builder for creating instances of [`RestApi`].
/// 
/// The `RestApiBuilder` provides a builder pattern for setting up a [`RestApi`] instance
/// by allowing users to specify optional configuration parameters such as the base URL,
/// user agent, and API key.
///
/// # Example
/// ```
/// use bothan_coingecko::api::RestApiBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let mut api = RestApiBuilder::default()
///         .with_url("https://api.coingecko.com/api/v3")
///         .with_api_key("your_api_key")
///         .with_user_agent("your_user_agent")
///         .build()
///         .unwrap();
/// }
/// ```
pub struct RestApiBuilder {
    /// Optional base URL of the CoinGecko REST API.
    url: Option<String>,
    /// HTTP user agent string to include in all API requests.
    user_agent: String,
    /// Optional API key for CoinGecko Pro access.
    api_key: Option<String>,
}

impl RestApiBuilder {
    /// Creates a new `RestApiBuilder` with the specified configuration.
    ///
    /// This method allows manual initialization of the builder using
    /// optional parameters for URL and API key, and a required user agent string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_coingecko::api::RestApiBuilder;
    ///
    /// let builder = RestApiBuilder::new(
    ///     Some("https://api.coingecko.com/api/v3"),
    ///     "custom-agent/1.0",
    ///     Some("your_api_key"),
    /// );
    /// ```
    pub fn new<T, U, V>(url: Option<T>, user_agent: U, api_key: Option<V>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        RestApiBuilder {
            url: url.map(|v| v.into()),
            user_agent: user_agent.into(),
            api_key: api_key.map(Into::into),
        }
    }

    /// Sets the URL for the API.
    /// 
    /// If not specified, the default URL is `DEFAULT_URL` when no API key is provided,
    /// and `DEFAULT_PRO_URL` when an API key is provided.
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the API key for the API.
    /// 
    /// The default is `None`.
    pub fn with_api_key<T: Into<String>>(mut self, api_key: T) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the user agent for the API.
    /// 
    /// The default is `DEFAULT_USER_AGENT`.
    pub fn with_user_agent<T: Into<String>>(mut self, user_agent: T) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Builds the [`RestApi`] instance.
    ///
    /// This method consumes the builder and attempts to create a fully configured client.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if:
    /// - The URL is invalid
    /// - The API key or user agent headers are malformed
    /// - The HTTP client fails to build
    pub fn build(self) -> Result<RestApi, BuildError> {
        let mut headers = HeaderMap::new();

        let agent = HeaderValue::from_str(&self.user_agent)?;
        headers.insert("User-Agent", agent);

        let url = match (&self.url, &self.api_key) {
            (Some(url), _) => url,
            (None, Some(_)) => DEFAULT_PRO_URL,
            (None, None) => DEFAULT_URL,
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
    /// Creates a new [`RestApiBuilder`] instance with default values.
    /// 
    /// This method creates a default builder with no custom URL or API key, 
    /// using the default user agent.
    /// 
    /// # Returns 
    /// A [`RestApiBuilder`] pre-configured with default values, suitable for further customization.
    fn default() -> Self {
        RestApiBuilder {
            url: None,
            api_key: None,
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }
}
