//! Builder for constructing Bitfinex REST API clients.
//!
//! This module provides the [`RestApiBuilder`] for creating configured instances of the Bitfinex REST API client.
//! It offers a fluent interface for setting configuration options such as the API URL before building the client.
//!
//! This module provides:
//!
//! - Fluent builder pattern for creating REST API clients
//! - URL configuration and validation
//! - HTTP client creation and configuration
//! - Error handling for invalid configurations

use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuildError;
use crate::api::rest::{DEFAULT_URL, RestApi};

/// A builder for creating configured Bitfinex REST API clients.
///
/// The `RestApiBuilder` provides a fluent interface for setting configuration options
/// before creating a `RestApi` instance. It handles URL parsing and HTTP client creation.
///
/// # Examples
///
/// ```rust
/// use bothan_bitfinex::api::builder::RestApiBuilder;
///
/// // Create a builder with default URL
/// let api = RestApiBuilder::default()
///     .build()
///     .expect("Failed to build API client");
///
/// // Create a builder with custom URL
/// let api = RestApiBuilder::new("https://api-pub.bitfinex.com/v2/")
///     .build()
///     .expect("Failed to build API client");
///
/// // Use the fluent interface
/// let api = RestApiBuilder::default()
///     .with_url("https://api-pub.bitfinex.com/v2/")
///     .build()
///     .expect("Failed to build API client");
/// ```
pub struct RestApiBuilder {
    /// The URL for the Bitfinex API.
    url: String,
}

impl RestApiBuilder {
    /// Creates a new `RestApiBuilder` with the specified URL.
    ///
    /// # Parameters
    ///
    /// - `url`: The URL for the Bitfinex API as a string
    ///
    /// # Returns
    ///
    /// A new `RestApiBuilder` instance with the specified URL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_bitfinex::api::builder::RestApiBuilder;
    ///
    /// let builder = RestApiBuilder::new("https://api-pub.bitfinex.com/v2/");
    /// ```
    pub fn new<T: Into<String>>(url: T) -> Self {
        RestApiBuilder { url: url.into() }
    }

    /// Sets the URL for the Bitfinex API.
    ///
    /// This method allows changing the URL after the builder has been created.
    /// It returns the builder instance for method chaining.
    ///
    /// # Parameters
    ///
    /// - `url`: The new URL for the Bitfinex API
    ///
    /// # Returns
    ///
    /// The builder instance with the updated URL for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_bitfinex::api::builder::RestApiBuilder;
    ///
    /// let builder = RestApiBuilder::default()
    ///     .with_url("https://api-pub.bitfinex.com/v2/");
    /// ```
    pub fn with_url<T: Into<String>>(mut self, url: T) -> Self {
        self.url = url.into();
        self
    }

    /// Builds a `RestApi` instance with the current configuration.
    ///
    /// This method validates the URL, creates an HTTP client, and returns a configured
    /// `RestApi` instance. If the URL is invalid or the HTTP client cannot be created,
    /// an error is returned.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `RestApi` instance on success,
    /// or a `BuildError` if the configuration is invalid.
    ///
    /// # Errors
    ///
    /// Returns a `BuildError` if:
    /// - The URL is invalid and cannot be parsed (`InvalidURL`)
    /// - The HTTP client cannot be created (`FailedToBuild`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_bitfinex::api::builder::RestApiBuilder;
    ///
    /// match RestApiBuilder::new("https://api-pub.bitfinex.com/v2/").build() {
    ///     Ok(api) => println!("API client created successfully"),
    ///     Err(e) => eprintln!("Failed to create API client: {}", e),
    /// }
    /// ```
    pub fn build(self) -> Result<RestApi, BuildError> {
        let parsed_url = Url::parse(&self.url)?;

        let client = ClientBuilder::new().build()?;

        Ok(RestApi::new(parsed_url, client))
    }
}

impl Default for RestApiBuilder {
    /// Creates a new `RestApiBuilder` with the default Bitfinex API URL.
    ///
    /// This method initializes the builder with the default Bitfinex API URL,
    /// which can then be customized using the builder methods.
    ///
    /// # Returns
    ///
    /// A new `RestApiBuilder` instance with the default URL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_bitfinex::api::builder::RestApiBuilder;
    ///
    /// let builder = RestApiBuilder::default();
    /// let api = builder.build().expect("Failed to build API client");
    /// ```
    fn default() -> Self {
        RestApiBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
