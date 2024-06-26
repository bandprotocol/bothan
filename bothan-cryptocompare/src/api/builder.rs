use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::ClientBuilder;
use url::Url;

use crate::api::error::BuilderError;
use crate::api::types::DEFAULT_URL;
use crate::api::CryptoCompareRestAPI;

/// Builder for creating instances of `CryptoCompareRestAPI`.
/// Methods can be chained to set the configuration values and the
/// service is constructed by calling the [`build`](CryptoCompareRestAPIBuilder::build) method.
/// # Example
/// ```no_run
/// use bothan_cryptocompare::api::CryptoCompareRestAPIBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     let mut builder = CryptoCompareRestAPIBuilder::default();
///     builder.set_url("https://min-api.cryptocompare.com/data");
///     builder.set_api_key("your_api_key");
///     let api = builder.build().unwrap();
///
///     // use api ...
/// }
/// ```
pub struct CryptoCompareRestAPIBuilder {
    url: String,
    api_key: Option<String>,
}

impl CryptoCompareRestAPIBuilder {
    /// Sets the URL for the API.
    /// The default URL is `DEFAULT_URL`.
    pub fn set_url(&mut self, url: &str) -> &Self {
        self.url = url.into();
        self
    }

    /// Sets the API key for the API.
    /// The default is `None`.
    pub fn set_api_key(&mut self, api_key: &str) -> &Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Builds the `CryptoCompareRestAPI` instance.
    pub fn build(self) -> Result<CryptoCompareRestAPI, BuilderError> {
        let mut headers = HeaderMap::new();

        let parsed_url = Url::parse(&self.url)?;

        if let Some(key) = &self.api_key {
            let mut val = HeaderValue::from_str(format!("Apikey {}", key).as_str())?;
            val.set_sensitive(true);
            headers.insert("authorization", val);
        }

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(CryptoCompareRestAPI::new(parsed_url, client))
    }
}

impl Default for CryptoCompareRestAPIBuilder {
    /// Creates a default `CryptoCompareRestAPIBuilder` instance with the default URL.
    fn default() -> Self {
        CryptoCompareRestAPIBuilder {
            url: DEFAULT_URL.to_string(),
            api_key: None,
        }
    }
}
