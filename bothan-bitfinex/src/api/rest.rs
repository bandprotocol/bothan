//! Bitfinex REST API client implementation.
//!
//! This module provides the [`RestApi`] for interacting with the Bitfinex REST API.
//! It enables fetching of market data, such as ticker information for both spot and funding markets,
//! and is used internally to implement the [`AssetInfoProvider`] trait for asset workers.
//!
//! This module provides:
//!
//! - HTTP client for making requests to Bitfinex REST endpoints
//! - Fetches ticker data for specified symbols via REST API calls
//! - Processes ticker responses and transforms them into [`AssetInfo`] for use in workers
//! - Handles error responses and data validation
//! - Supports both spot and funding ticker data retrieval

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider;
use reqwest::{Client, Url};
use rust_decimal::Decimal;

use crate::api::error::ProviderError;
use crate::api::msg::ticker::Ticker;

pub const DEFAULT_URL: &str = "https://api-pub.bitfinex.com/v2/";

/// A client for interacting with the Bitfinex REST API.
///
/// The `RestApi` provides methods to fetch ticker data from the Bitfinex API
/// and implements the [`AssetInfoProvider`] trait for integration with asset workers.
///
/// # Examples
///
/// ```rust
/// use bothan_bitfinex::api::rest::RestApi;
/// use reqwest::Client;
/// use url::Url;
///
/// let url = Url::parse("https://api-pub.bitfinex.com/v2/").unwrap();
/// let client = Client::new();
/// let api = RestApi::new(url, client);
///
/// // Fetch ticker data for specific symbols
/// let tickers = vec!["tBTCUSD", "tETHUSD"];
/// let result = api.get_tickers(&tickers).await;
/// ```
pub struct RestApi {
    /// The base URL for the Bitfinex API.
    url: Url,
    /// The HTTP client for making requests.
    client: Client,
}

impl RestApi {
    /// Creates a new `RestApi` instance with the specified URL and HTTP client.
    ///
    /// # Parameters
    ///
    /// - `url`: The base URL for the Bitfinex API
    /// - `client`: The HTTP client for making requests
    ///
    /// # Returns
    ///
    /// A new `RestApi` instance configured with the provided URL and client.
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    /// Fetches ticker data for the specified symbols from the Bitfinex API.
    ///
    /// This method makes a REST API call to the Bitfinex tickers endpoint to retrieve
    /// current market data for the specified trading pairs. The response includes
    /// both spot and funding ticker information depending on the symbol type.
    ///
    /// # Parameters
    ///
    /// - `tickers`: A slice of symbol identifiers to fetch ticker data for
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `Ticker` instances on success,
    /// or a `reqwest::Error` if the request fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_bitfinex::api::rest::RestApi;
    /// use reqwest::Client;
    /// use url::Url;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = Url::parse("https://api-pub.bitfinex.com/v2/").unwrap();
    ///     let client = Client::new();
    ///     let api = RestApi::new(url, client);
    ///
    ///     let symbols = vec!["tBTCUSD", "fUSD"];
    ///     let tickers = api.get_tickers(&symbols).await?;
    ///
    ///     for ticker in tickers {
    ///         println!("Symbol: {}, Price: {}", ticker.symbol(), ticker.price());
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `reqwest::Error` if:
    /// - The HTTP request fails due to network issues
    /// - The API returns an error response
    /// - The response cannot be parsed as JSON
    pub async fn get_tickers<T: AsRef<str>>(
        &self,
        tickers: &[T],
    ) -> Result<Vec<Ticker>, reqwest::Error> {
        let url = format!("{}/tickers", self.url);
        let symbols = tickers
            .iter()
            .map(|t| t.as_ref())
            .collect::<Vec<&str>>()
            .join(",");
        let params = vec![("symbols", symbols)];

        let resp = self.client.get(&url).query(&params).send().await?;
        resp.error_for_status_ref()?;
        resp.json().await
    }
}

#[async_trait::async_trait]
impl AssetInfoProvider for RestApi {
    type Error = ProviderError;

    /// Fetches asset information for the specified asset IDs.
    ///
    /// This method retrieves ticker data from the Bitfinex API for the given asset IDs
    /// and transforms the data into [`AssetInfo`] instances for use in asset workers.
    /// The method handles both spot and funding ticker types automatically.
    ///
    /// # Parameters
    ///
    /// - `ids`: A slice of asset identifiers to fetch information for
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `AssetInfo` instances on success,
    /// or a `ProviderError` if the request fails or contains invalid data.
    ///
    /// # Errors
    ///
    /// Returns a `ProviderError` if:
    /// - The API request fails (`RequestError`)
    /// - The ticker data contains invalid values such as NaN (`InvalidValue`)
    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error> {
        let timestamp = chrono::Utc::now().timestamp();
        self.get_tickers(ids)
            .await?
            .into_iter()
            .map(|t| {
                let price =
                    Decimal::from_f64_retain(t.price()).ok_or(ProviderError::InvalidValue)?;
                Ok(AssetInfo::new(t.symbol().to_string(), price, timestamp))
            })
            .collect()
    }
}
