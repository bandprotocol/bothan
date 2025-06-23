//! CoinMarketCap REST API client implementation.
//!
//! This module provides the [`RestApi`], a client for interacting with the CoinMarketCap REST API.
//! It includes methods for retrieving asset quotes and is used internally
//! to implement the [`AssetInfoProvider`] trait for asset workers.
//!
//! This module provides:
//!
//! - Fetches the latest quotes for assets from the `/v2/cryptocurrency/quotes/latest` endpoint
//! - Transforms API responses into [`AssetInfo`] for use in workers
//! - Handles deserialization and error propagation

use std::collections::HashMap;

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider;
use itertools::Itertools;
use reqwest::{Client, Url};
use rust_decimal::Decimal;

use crate::api::error::ParseError;
use crate::api::types::{Quote, Response as CmcResponse};
use crate::worker::error::ProviderError;

/// Client for interacting with the CoinMarketCap REST API.
///
/// The [`RestApi`] includes a base URL and HTTP client used to send
/// requests to the CoinMarketCap REST API. It provides methods for fetching asset quotes. It is also used to implement the [`AssetInfoProvider`] trait
/// for integration into the REST API worker.
///
/// # Examples
///
/// ```rust
/// use bothan_coinmarketcap::api::{RestApi, types::Quote};
/// use reqwest::ClientBuilder;
/// use reqwest::header::{HeaderMap, HeaderValue};
/// use url::Url;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut headers = HeaderMap::new();
///     let api_key = HeaderValue::from_str("your_api_key")?;
///     headers.insert("X-CMC_PRO_API_KEY", api_key);
///
///     let client = ClientBuilder::new().default_headers(headers).build()?;
///
///     let api = RestApi::new(Url::parse("https://pro-api.coinmarketcap.com")?, client);
///     Ok(())
/// }
/// ```
pub struct RestApi {
    /// The base URL of the CoinMarketCap REST API.
    url: Url,
    /// The reqwest HTTP client used to make requests.
    client: Client,
}

impl RestApi {
    /// Creates a new instance of `CoinMarketCapRestAPI`.
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    /// Retrieves market data for the specified cryptocurrency IDs from the CoinMarketCap REST API.
    ///
    /// This method constructs a request to the CoinMarketCap `/v2/cryptocurrency/quotes/latest` endpoint
    /// and returns a vector of `Option<Quote>`, where each element corresponds to the ID at the same
    /// position in the input slice. If a quote is not found for a given ID, or if an ID appears
    /// more than once, `None` will be returned in that position.
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
    pub async fn get_latest_quotes(
        &self,
        ids: &[u64],
    ) -> Result<Vec<Option<Quote>>, reqwest::Error> {
        let url = format!("{}v2/cryptocurrency/quotes/latest", self.url);
        let ids_string = ids.iter().map(|id| id.to_string()).join(",");
        let params = vec![("id", ids_string)];

        let request_builder = self.client.get(&url).query(&params);
        let response = request_builder.send().await?.error_for_status()?;
        let cmc_response = response
            .json::<CmcResponse<HashMap<String, Quote>>>()
            .await?;
        let mut quote_map = cmc_response.data;

        let quotes = ids
            .iter()
            .map(|id| quote_map.remove(&id.to_string()))
            .collect();
        Ok(quotes)
    }
}

#[async_trait::async_trait]
impl AssetInfoProvider for RestApi {
    type Error = ProviderError;

    /// Fetches asset information for a list of coin IDs from the CoinMarketCap REST API.
    ///
    /// This method retrieves current quotes for the given `ids` by calling
    /// [`RestApi::get_latest_quotes`] and transforms the results into a vector of [`AssetInfo`] structs.
    ///
    /// Each entry in the response is converted into an [`AssetInfo`] instance using:
    /// - The coin ID as the asset identifier
    /// - The USD price converted into a [`Decimal`] using `from_f64_retain`
    /// - The last update timestamp returned by the API
    ///
    /// # Errors
    ///
    /// Returns a [`ProviderError`] if:
    /// - The HTTP request fails or returns an invalid response
    /// - The price contains a value that cannot be converted into a valid `Decimal`
    ///
    /// [`RestApi::get_latest_quotes`]: crate::api::RestApi::get_latest_quotes
    /// [`AssetInfo`]: bothan_lib::types::AssetInfo
    /// [`Decimal`]: rust_decimal::Decimal
    /// [`ProviderError`]: crate::worker::error::ProviderError
    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error> {
        let int_ids = ids
            .iter()
            .map(|id| {
                id.parse::<u64>()
                    .map_err(|_| ProviderError::InvalidId(id.clone()))
            })
            .collect::<Result<Vec<u64>, _>>()?;

        let asset_info = self
            .get_latest_quotes(&int_ids)
            .await?
            .into_iter()
            .filter_map(|quote| quote.and_then(|q| parse_quote(q).ok()))
            .collect();

        Ok(asset_info)
    }
}

/// Parses a `Quote` into an [`AssetInfo`] struct.
fn parse_quote(quote: Quote) -> Result<AssetInfo, ParseError> {
    let price_float = quote
        .price_quotes
        .usd
        .price
        .ok_or(ParseError::MissingPrice)?;
    let price = Decimal::from_f64_retain(price_float).ok_or(ParseError::InvalidPrice)?;
    let ts = quote
        .price_quotes
        .usd
        .last_updated
        .parse::<chrono::DateTime<chrono::Utc>>()?
        .timestamp();

    Ok(AssetInfo::new(quote.id.to_string(), price, ts))
}

#[cfg(test)]
pub(crate) mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use super::*;
    use crate::api::RestApiBuilder;
    use crate::api::types::{PriceQuote, PriceQuotes, Status};

    pub(crate) async fn setup() -> (ServerGuard, RestApi) {
        let server = Server::new_async().await;

        let builder = RestApiBuilder::default()
            .with_url(&server.url())
            .with_api_key("test");
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) fn mock_quote() -> Quote {
        Quote {
            id: 1,
            symbol: "BTC".to_string(),
            slug: "bitcoin".to_string(),
            name: "Bitcoin".to_string(),
            price_quotes: PriceQuotes {
                usd: PriceQuote {
                    price: Some(80000.0),
                    volume_24h: 123.0,
                    volume_change_24h: 456.0,
                    market_cap: Some(10000000.0),
                    market_cap_dominance: 99.0,
                    fully_diluted_market_cap: 2100000.0,
                    percent_change_1h: 12.9,
                    percent_change_24h: -70.0,
                    percent_change_7d: -2.0,
                    percent_change_30d: -1.0,
                    last_updated: "2024-03-16T06:55:15.626Z".to_string(),
                },
            },
        }
    }

    pub(crate) trait MockCoinMarketCap {
        fn set_successful_quotes(&mut self, ids: &[&str], quotes: &[Quote]) -> Mock;
        fn set_arbitrary_quotes<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_quotes(&mut self, ids: &[&str]) -> Mock;
    }

    impl MockCoinMarketCap for ServerGuard {
        fn set_successful_quotes(&mut self, ids: &[&str], quotes: &[Quote]) -> Mock {
            let response = CmcResponse::<HashMap<String, Quote>> {
                data: quotes
                    .iter()
                    .map(|quote| (quote.id.to_string(), quote.clone()))
                    .collect(),
                status: Status {
                    timestamp: "100000".to_string(),
                    error_code: 0,
                    error_message: None,
                    elapsed: 1,
                    credit_count: 1,
                    notice: None,
                },
            };
            self.mock("GET", "/v2/cryptocurrency/quotes/latest")
                .match_query(Matcher::UrlEncoded("id".into(), ids.join(",")))
                .with_status(200)
                .with_body(serde_json::to_string(&response).unwrap())
                .create()
        }

        fn set_arbitrary_quotes<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock {
            self.mock("GET", "/v2/cryptocurrency/quotes/latest")
                .match_query(Matcher::UrlEncoded("id".into(), ids.join(",")))
                .with_status(200)
                .with_body(data)
                .create()
        }

        fn set_failed_quotes(&mut self, ids: &[&str]) -> Mock {
            self.mock("GET", "/v2/cryptocurrency/quotes/latest")
                .match_query(Matcher::UrlEncoded("id".into(), ids.join(",")))
                .with_status(500)
                .create()
        }
    }

    #[tokio::test]
    async fn test_successful_get_quotes() {
        let (mut server, client) = setup().await;

        let quotes = vec![mock_quote()];
        let mock = server.set_successful_quotes(&["1"], &quotes);

        let result = client.get_latest_quotes(&[1]).await;
        let expected_result = quotes.into_iter().map(Some).collect::<Vec<Option<Quote>>>();
        mock.assert();
        assert_eq!(result.unwrap(), expected_result);
    }

    #[tokio::test]
    async fn test_successful_get_quotes_with_missing_data() {
        let (mut server, client) = setup().await;

        let quotes = vec![mock_quote()];

        let mock = server.set_successful_quotes(&["1", "0"], &quotes);

        let result = client.get_latest_quotes(&[1, 0]).await;

        mock.assert();
        let expected_result = vec![Some(quotes[0].clone()), None];
        assert_eq!(result.unwrap(), expected_result);
    }

    #[tokio::test]
    async fn test_get_quotes_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let mock = server.set_arbitrary_quotes(&["1"], "abc");

        let result = client.get_latest_quotes(&[1]).await;

        mock.assert();

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_failed_get_quotes() {
        let (mut server, client) = setup().await;
        let mock = server.set_failed_quotes(&["1"]);

        let result = client.get_latest_quotes(&[1]).await;
        mock.assert();
        assert!(result.is_err());
    }
}
