//! Band REST API client implementation.
//!
//! This module provides the [`RestApi`], a client for interacting with the Band REST API.
//! It includes methods for retrieving asset quotes and is used internally
//! to implement the [`AssetInfoProvider`] trait for asset workers.
//!
//! This module provides:
//!
//! - Fetches the latest quotes for assets from the `/v2/cryptocurrency/quotes/latest` endpoint
//! - Transforms API responses into [`AssetInfo`] for use in workers
//! - Handles deserialization and error propagation

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider;
use itertools::Itertools;
use reqwest::{Client, Url};
use rust_decimal::Decimal;

use crate::api::error::{ParseError, ProviderError};
use crate::api::types::Price;

/// Client for interacting with the Band REST API.
///
/// The [`RestApi`] includes a base URL and HTTP client used to send
/// requests to the Band REST API. It provides methods for fetching asset quotes. It is also used to implement the [`AssetInfoProvider`] trait
/// for integration into the REST API worker.
///
/// # Examples
///
/// ```rust
/// use bothan_band::api::{RestApi, types::Quote};
/// use reqwest::ClientBuilder;
/// use reqwest::header::{HeaderMap, HeaderValue};
/// use url::Url;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut headers = HeaderMap::new();
///     let client = ClientBuilder::new().default_headers(headers).build()?;
///
///     let api = RestApi::new(Url::parse("https://bandsource-url.com")?, client);
///     Ok(())
/// }
/// ```
pub struct RestApi {
    /// The base URL of the Band REST API.
    url: Url,
    /// The reqwest HTTP client used to make requests.
    client: Client,
}

impl RestApi {
    /// Creates a new instance of `BandRestAPI`.
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    /// Retrieves market data for the specified cryptocurrency IDs from the Band REST API.
    ///
    /// This method constructs a request to the Band endpoint
    /// and returns a vector of `Price<...>`, where each element corresponds to the ID at the same
    /// position in the input slice.
    ///
    /// # Query Construction
    ///
    /// The query includes:
    /// - `id`: comma-separated list of coin IDs
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if:
    /// - The request fails due to network issues
    /// - The response status is not 2xx
    /// - JSON deserialization into `HashMap<String, Quote>` fails
    pub async fn get_latest_prices(&self, ids: &[String]) -> Result<Vec<Price>, reqwest::Error> {
        let url = format!("{}prices/", self.url);
        let ids_string = ids.iter().map(|id| id.to_string()).join(",");
        let params = vec![("signals", ids_string)];

        let request_builder = self.client.get(&url).query(&params);
        let response = request_builder.send().await?.error_for_status()?;
        let prices = response.json::<Vec<Price>>().await?;

        Ok(prices)
    }
}

#[async_trait::async_trait]
impl AssetInfoProvider for RestApi {
    type Error = ProviderError;

    /// Fetches asset information for a list of coin IDs from the Band REST API.
    ///
    /// This method retrieves current prices for the given `ids` by calling
    /// [`RestApi::get_latest_prices`] and transforms the results into a vector of [`AssetInfo`] structs.
    ///
    /// Each entry in the response is converted into an [`AssetInfo`] instance using:
    /// - The coin signal as the asset identifier
    /// - The price and timestamp returned by the API
    ///
    /// # Errors
    ///
    /// Returns a [`ProviderError`] if:
    /// - The HTTP request fails or returns an invalid response
    /// - The price contains a value that cannot be converted into a valid `Decimal`
    ///
    /// [`RestApi::get_latest_prices`]: crate::api::RestApi::get_latest_prices
    /// [`AssetInfo`]: bothan_lib::types::AssetInfo
    /// [`Decimal`]: rust_decimal::Decimal
    /// [`ProviderError`]: crate::worker::error::ProviderError
    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error> {
        let asset_info = self
            .get_latest_prices(ids)
            .await?
            .into_iter()
            .map(parse_price)
            .filter_map(|price| price.ok())
            .collect();

        Ok(asset_info)
    }
}

/// Parses a `Price` into an [`AssetInfo`] struct.
fn parse_price(band_price: Price) -> Result<AssetInfo, ParseError> {
    let price = Decimal::from_f64_retain(band_price.price).ok_or(ParseError::InvalidPrice)?;
    let ts = band_price.timestamp;
    Ok(AssetInfo::new(band_price.signal, price, ts))
}
