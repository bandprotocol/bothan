use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::HtxRestAPI;

/// Builder for creating instances of `HtxRestAPI`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](HtxRestAPIBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_htx::api::HtxRestAPIBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let api = HtxRestAPIBuilder::default()
///         .with_url("https://api.htx.com/")
///         .build()
///         .unwrap();
///
///     // use api ...
/// }
/// ```
pub struct HtxRestAPIBuilder {
    url: String,
}

impl HtxRestAPIBuilder {
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
        self.url = url.into();
        self
    }

    /// Builds the `HtxRestAPI` instance.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the instance was created successfully, or a `BuilderError` if there was a problem.
    pub fn build(self) -> Result<HtxRestAPI, BuilderError> {
        let parsed_url = Url::parse(&self.url)?;

        let client = ClientBuilder::new().build()?;

        Ok(HtxRestAPI::new(parsed_url, client))
    }
}

impl Default for HtxRestAPIBuilder {
    /// Creates a default `HtxRestAPIBuilder` instance with default values.
    fn default() -> Self {
        HtxRestAPIBuilder {
            url: DEFAULT_URL.into(),
        }
    }
}
