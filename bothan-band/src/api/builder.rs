//! Builder for configuring and constructing `BandAPI`.
//!
//! This module provides a builder for constructing [`RestApi`] clients used
//! to interact with the Band REST API. The builder supports optional configuration
//! of base URL and API key.
//!
//! The module provides:
//!
//! - The [`RestApiBuilder`] for REST API building
//! - Supports setting the API base URL and API key
//! - Automatically uses the default Band URL when parameters are omitted during the [`build`](`RestApiBuilder::build`) call

use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap};
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
/// use bothan_band::api::RestApiBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let mut api = RestApiBuilder::default()
///         .with_url("https://bandsource-url.com")
///         .build()
///         .unwrap();
/// }
/// ```
pub struct RestApiBuilder {
    /// Base URL of the Band REST API.
    url: String,
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
    /// use bothan_band::api::RestApiBuilder;
    ///
    /// let builder = RestApiBuilder::new(
    ///     "https://bandsource-url.com",
    /// );
    /// ```
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        RestApiBuilder {
            url: url.into(),
        }
    }

    /// Sets the URL for the Band API.
    /// The default URL is `DEFAULT_URL`.
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.into();
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
    /// - The API key is missing (required for Band)
    pub fn build(self) -> Result<RestApi, BuildError> {
        let headers = HeaderMap::new();

        let parsed_url = Url::parse(&self.url)?;

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(RestApi::new(parsed_url, client))
    }
}

impl Default for RestApiBuilder {
    /// Creates a new `BandRestAPIBuilder` with the
    /// default URL and no API key.
    fn default() -> Self {
        RestApiBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
