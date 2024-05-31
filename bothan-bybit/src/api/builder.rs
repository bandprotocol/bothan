use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::BybitRestAPI;

/// Builder for creating instances of `BybitRestAPI`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](BybitRestAPIBuilder::build) method.
///
/// # Example
/// ```no_run
/// use bothan_bybit::api::BybitRestAPIBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let api = BybitRestAPIBuilder::default()
///         .with_url("https://api.bybit.com")
///         .build()
///         .unwrap();
///
///     // use api ...
/// }
/// ```
pub struct BybitRestAPIBuilder {
    url: String,
}

impl BybitRestAPIBuilder {
    /// Sets the URL for the api.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL.
    ///
    /// # Returns
    ///
    /// A mutable reference to the builder.
    pub fn with_url(&mut self, url: &str) -> &Self {
        self.url = url.into();
        self
    }

    /// Builds the `BybitRestAPI` instance.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the instance was created successfully, or a `BuilderError` if there was a problem.
    pub fn build(self) -> Result<BybitRestAPI, BuilderError> {
        let parsed_url = Url::parse(&self.url)?;
        let client = ClientBuilder::new().build()?;

        Ok(BybitRestAPI::new(parsed_url, client))
    }
}

impl Default for BybitRestAPIBuilder {
    /// Creates a default `BybitRestAPIBuilder` instance with the default URL.
    fn default() -> Self {
        BybitRestAPIBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
