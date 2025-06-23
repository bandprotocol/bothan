//! Builder for configuring and constructing `CoinMarketCapRestAPI`.
//!
//! This module provides a builder for constructing [`RestApi`] clients used
//! to interact with the CoinMarketCap REST API. The builder supports optional configuration
//! of base URL and API key.
//!
//! The module provides:
//!
//! - The [`RestApiBuilder`] for REST API building
//! - Supports setting the API base URL and API key
//! - Automatically uses the default CoinMarketCap URL when parameters are omitted during the [`build`](`RestApiBuilder::build`) call

use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use url::Url;

use crate::api::RestApi;
use crate::api::error::BuildError;
use crate::api::types::DEFAULT_URL;

/// Builder for creating instances of [`RestApi`].
///
/// The `RestApiBuilder` provides a builder pattern for setting up a [`RestApi`] instance
/// by allowing users to specify optional configuration parameters such as the base URL and API key.
///
/// # Example
/// ```
/// use bothan_coinmarketcap::api::RestApiBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let mut api = RestApiBuilder::default()
///         .with_url("https://pro-api.coinmarketcap.com")
///         .with_api_key("your_api_key")
///         .build()
///         .unwrap();
/// }
/// ```
pub struct RestApiBuilder {
    /// Base URL of the CoinMarketCap REST API.
    url: String,
    /// Optional API key for CoinMarketCap.
    api_key: Option<String>,
}

impl RestApiBuilder {
    /// Creates a new `RestApiBuilder` with the specified configuration.
    ///
    /// This method allows manual initialization of the builder using
    /// optional parameter for API key, and a required URL string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_coinmarketcap::api::RestApiBuilder;
    ///
    /// let builder = RestApiBuilder::new(
    ///     "https://pro-api.coinmarketcap.com",
    ///     Some("your_api_key"),
    /// );
    /// ```
    pub fn new<T, U>(url: T, api_key: Option<U>) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        RestApiBuilder {
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

    /// Builds the [`RestApi`] instance.
    ///
    /// This method consumes the builder and attempts to create a fully configured client.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if:
    /// - The URL is invalid
    /// - The API key or HTTP headers are malformed
    /// - The HTTP client fails to build
    /// - The API key is missing (required for CoinMarketCap)
    pub fn build(self) -> Result<RestApi, BuildError> {
        let mut headers = HeaderMap::new();

        let parsed_url = Url::parse(&self.url)?;

        let api_key = self.api_key.ok_or(BuildError::MissingAPIKey)?;

        let mut val = HeaderValue::from_str(&api_key)?;
        val.set_sensitive(true);
        headers.insert("X-CMC_PRO_API_KEY", val);

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(RestApi::new(parsed_url, client))
    }
}

impl Default for RestApiBuilder {
    /// Creates a new `CoinMarketCapRestAPIBuilder` with the
    /// default URL and no API key.
    fn default() -> Self {
        RestApiBuilder {
            url: DEFAULT_URL.into(),
            api_key: None,
        }
    }
}
