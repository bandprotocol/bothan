//! Band REST API client implementation.
//!
//! This module provides the [`RestApi`], a client for interacting with the Band REST API.
//! It includes methods for retrieving asset quotes and is used internally
//! to implement the [`AssetInfoProvider`] trait for asset workers.
//!
//! This module provides:
//!
//! - Fetches the latest quotes for assets from the Band `/prices` endpoint
//! - Transforms API responses into [`AssetInfo`] for use in workers
//! - Handles deserialization and error propagation

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider;
use itertools::Itertools;
use reqwest::{Client, Url};
use rust_decimal::Decimal;
use tracing::{error, warn};

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
/// use bothan_band::api::{RestApi, types::Price};
/// use reqwest::ClientBuilder;
/// use url::Url;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = ClientBuilder::new().build()?;
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
    /// and returns a vector of `Price`, where each element corresponds to the ID at the same
    /// position in the input slice.
    ///
    /// # Query Construction
    ///
    /// The query includes:
    /// - `signals`: comma-separated list of coin IDs
    ///
    /// # Errors
    ///
    /// Returns a [`ProviderError`] if:
    /// - The request fails due to network issues
    /// - The response status is not 2xx
    /// - JSON deserialization into `Vec<Price>` fails
    pub async fn get_latest_prices(&self, ids: &[String]) -> Result<Vec<Price>, ProviderError> {
        let url = format!("{}prices", self.url);
        let ids_string = ids.iter().map(String::as_str).join(",");
        let params = vec![("signals", &ids_string)];

        let resp = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|error| ProviderError::SendingRequestError {
                error,
                signals: ids_string.clone(),
            })?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp
                .text()
                .await
                .unwrap_or_else(|err| format!("failed to read response body: {err}"));
            return Err(ProviderError::HttpStatusError {
                status,
                body,
                signals: ids_string.clone(),
            });
        }

        resp.json::<Vec<Price>>()
            .await
            .map_err(|source| ProviderError::ParseResponseError {
                source,
                signals: ids_string,
            })
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
        let prices = self.get_latest_prices(ids).await?;
        let mut asset_info = Vec::with_capacity(prices.len());

        for band_price in prices {
            match parse_price(band_price) {
                Ok(info) => asset_info.push(info),
                Err(ParseError::InvalidPrice { price, signal }) => {
                    error!("failed to parse price '{price}' for signal '{signal}'");
                }
                Err(ParseError::MissingPrice(signal)) => {
                    warn!("missing price for '{signal}'");
                }
                Err(ParseError::MissingTimestamp(signal)) => {
                    warn!("missing timestamp for '{signal}'");
                }
            }
        }

        Ok(asset_info)
    }
}

/// Parses a `Price` into an [`AssetInfo`] struct.
fn parse_price(band_price: Price) -> Result<AssetInfo, ParseError> {
    let signal = band_price.signal;
    let price = band_price
        .price
        .ok_or(ParseError::MissingPrice(signal.clone()))?;
    let price = Decimal::from_f64_retain(price).ok_or(ParseError::InvalidPrice {
        price,
        signal: signal.clone(),
    })?;
    let ts = band_price
        .timestamp
        .ok_or(ParseError::MissingTimestamp(signal.clone()))?;
    Ok(AssetInfo::new(signal, price, ts))
}

#[cfg(test)]
mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use super::*;
    use crate::api::RestApiBuilder;
    use crate::api::types::Price;

    // Setup a test server and RestApi client instance
    async fn setup() -> (ServerGuard, RestApi) {
        let server = Server::new_async().await;
        let builder = RestApiBuilder::new(server.url());
        let api = builder.build().unwrap();
        (server, api)
    }

    fn mock_price(signal: &str, price: f64, timestamp: i64) -> Price {
        Price {
            signal: signal.to_string(),
            price: Some(price),
            timestamp: Some(timestamp),
        }
    }

    fn mock_price_none(signal: &str, timestamp: i64) -> Price {
        Price {
            signal: signal.to_string(),
            price: None,
            timestamp: Some(timestamp),
        }
    }

    trait MockBandRest {
        fn set_successful_prices(&mut self, ids: &[String], prices: &[Price]) -> Mock;
        fn set_arbitrary_prices<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[String],
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_prices(&mut self, ids: &[String]) -> Mock;
    }

    impl MockBandRest for ServerGuard {
        fn set_successful_prices(&mut self, ids: &[String], prices: &[Price]) -> Mock {
            let response = serde_json::to_string(prices).unwrap();
            self.mock("GET", "/prices")
                .match_query(Matcher::UrlEncoded("signals".into(), ids.join(",")))
                .with_status(200)
                .with_body(response)
                .create()
        }

        fn set_arbitrary_prices<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[String],
            data: StrOrBytes,
        ) -> Mock {
            self.mock("GET", "/prices")
                .match_query(Matcher::UrlEncoded("signals".into(), ids.join(",")))
                .with_status(200)
                .with_body(data)
                .create()
        }

        fn set_failed_prices(&mut self, ids: &[String]) -> Mock {
            self.mock("GET", "/prices")
                .match_query(Matcher::UrlEncoded("signals".into(), ids.join(",")))
                .with_status(500)
                .create()
        }
    }

    #[tokio::test]
    async fn test_successful_get_latest_prices() {
        let (mut server, client) = setup().await;

        let ids = vec!["BTC".to_string()];
        let prices = vec![mock_price("BTC", 80000.0, 100000)];
        let mock = server.set_successful_prices(&ids, &prices);

        let result = client.get_latest_prices(&ids).await;
        mock.assert();
        assert_eq!(result.unwrap(), prices);
    }

    #[tokio::test]
    async fn test_successful_get_latest_prices_with_multiple_assets() {
        let (mut server, client) = setup().await;

        let ids = vec!["BTC".to_string(), "ETH".to_string()];
        let prices = vec![
            mock_price("BTC", 80000.0, 100000),
            mock_price("ETH", 3500.0, 100002),
        ];
        let mock = server.set_successful_prices(&ids, &prices);

        let result = client.get_latest_prices(&ids).await;
        mock.assert();
        assert_eq!(result.unwrap(), prices);
    }

    #[tokio::test]
    async fn test_get_latest_prices_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let ids = vec!["BTC".to_string()];
        let mock = server.set_arbitrary_prices(&ids, "not valid json");

        let result = client.get_latest_prices(&ids).await;
        mock.assert();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_failed_get_latest_prices() {
        let (mut server, client) = setup().await;
        let ids = vec!["BTC".to_string()];
        let mock = server.set_failed_prices(&ids);

        let result = client.get_latest_prices(&ids).await;
        mock.assert();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_asset_info_price_is_none() {
        let (mut server, client) = setup().await;

        let ids = vec!["BTC".to_string(), "ETH".to_string()];
        let prices = vec![
            mock_price("BTC", 80000.0, 100000),
            // ETH will have price = None
            mock_price_none("ETH", 100002),
        ];
        let mock = server.set_successful_prices(&ids, &prices);

        let asset_infos = client.get_asset_info(&ids).await.unwrap();

        mock.assert();

        // Only BTC info should be present; ETH is skipped due to None price
        assert_eq!(asset_infos.len(), 1);
        assert_eq!(asset_infos[0].id, "BTC");
        assert_eq!(
            asset_infos[0].price,
            rust_decimal::Decimal::from_f64_retain(80000.0).unwrap()
        );
        assert_eq!(asset_infos[0].timestamp, 100000);
    }

    #[tokio::test]
    async fn test_get_asset_info_success_multiple_assets() {
        let (mut server, client) = setup().await;

        let ids = vec!["BTC".to_string(), "ETH".to_string(), "BAND".to_string()];
        let prices = vec![
            mock_price("BTC", 80000.0, 100000),
            mock_price("ETH", 3500.0, 100002),
            mock_price("BAND", 1.6, 100003),
        ];
        let mock = server.set_successful_prices(&ids, &prices);

        let asset_infos = client.get_asset_info(&ids).await.unwrap();

        mock.assert();

        assert_eq!(asset_infos.len(), 3);

        assert_eq!(asset_infos[0].id, "BTC");
        assert_eq!(
            asset_infos[0].price,
            rust_decimal::Decimal::from_f64_retain(80000.0).unwrap()
        );
        assert_eq!(asset_infos[0].timestamp, 100000);

        assert_eq!(asset_infos[1].id, "ETH");
        assert_eq!(
            asset_infos[1].price,
            rust_decimal::Decimal::from_f64_retain(3500.0).unwrap()
        );
        assert_eq!(asset_infos[1].timestamp, 100002);

        assert_eq!(asset_infos[2].id, "BAND");
        assert_eq!(
            asset_infos[2].price,
            rust_decimal::Decimal::from_f64_retain(1.6).unwrap()
        );
        assert_eq!(asset_infos[2].timestamp, 100003);
    }
}
