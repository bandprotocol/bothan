//! CoinGecko REST API client implementation.
//!
//! This module provides the [`RestApi`], a client for interacting with the CoinGecko REST API.
//! It includes methods for retrieving coin metadata and current USD prices, and is used internally
//! to implement the [`AssetInfoProvider`] trait for asset workers.
//! 
//! This module provides:
//!
//! - Fetches the full list of coins from the `/coins/list` endpoint
//! - Retrieves simple price data from the `/simple/price` endpoint
//! - Transforms API responses into [`AssetInfo`] for use in workers
//! - Handles deserialization and error propagation
use std::collections::HashMap;

use bothan_lib::types::AssetInfo;
use bothan_lib::worker::rest::AssetInfoProvider;
use reqwest::{Client, RequestBuilder, Url};
use rust_decimal::Decimal;
use serde::de::DeserializeOwned;

use crate::api::error::ProviderError;
use crate::api::types::{Coin, Price};

/// Client for interacting with the CoinGecko REST API.
///
/// The [`RestApi`] includes a base URL and HTTP client used to send
/// requests to the CoinGecko REST API. It provides methods for fetching coin lists and
/// current USD prices. It is also used to implement the [`AssetInfoProvider`] trait
/// for integration into the REST API worker.
///
/// # Examples
///
/// ```rust
/// use bothan_coingecko::api::{RestApi, types::Price};
/// use reqwest::ClientBuilder;
/// use reqwest::header::{HeaderMap, HeaderValue};
/// use url::Url;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut headers = HeaderMap::new();
///     let agent = HeaderValue::from_str("bothan")?;
///     headers.insert("User-Agent", agent);
///
///     let client = ClientBuilder::new().default_headers(headers).build()?;
/// 
///     let api = RestApi::new(Url::parse("https://api.coingecko.com/api/v3/")?, client);
///     Ok(())
/// }
/// ```
pub struct RestApi {
    /// The base URL of the CoinGecko REST API.
    url: Url,
    /// The reqwest HTTP client used to make requests.
    client: Client,
}

impl RestApi {
    /// Creates a new instance of `CoinGeckoRestAPI`.
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    /// Retrieves a list of coins from the CoinGecko REST API.
    pub async fn get_coins_list(&self) -> Result<Vec<Coin>, reqwest::Error> {
        let url = format!("{}coins/list", self.url);
        let builder = self.client.get(url);

        request::<Vec<Coin>>(builder).await
    }

    /// Retrieves market data for the specified coins from the CoinGecko REST API.
    /// 
    /// This method constructs a request to the CoinGecko `/simple/price` endpoint
    /// and returns a mapping from coin IDs to [`Price`] structures containing the
    /// latest USD price and timestamp.
    ///
    /// # Query Construction
    ///
    /// The query includes:
    /// - `ids`: comma-separated list of coin IDs
    /// - `vs_currencies`: always `"usd"`
    /// - `include_last_updated_at`: `"true"`
    /// - `precision`: `"full"`
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if:
    /// - The request fails due to network issues
    /// - The response status is not 2xx
    /// - JSON deserialization into [`HashMap<String, Price>`] fails
    pub async fn get_simple_price_usd<T: AsRef<str>>(
        &self,
        ids: &[T],
    ) -> Result<HashMap<String, Price>, reqwest::Error> {
        let url = format!("{}simple/price", self.url);
        let joined_ids = ids
            .iter()
            .map(|id| id.as_ref())
            .collect::<Vec<&str>>()
            .join(",");

        let params = vec![
            ("vs_currencies", "usd"),
            ("ids", &joined_ids),
            ("include_last_updated_at", "true"),
            ("precision", "full"),
        ];

        let builder_with_query = self.client.get(&url).query(&params);

        request::<HashMap<String, Price>>(builder_with_query).await
    }
}

/// Sends an HTTP request and deserializes the JSON response.
///
/// This function executes the given [`RequestBuilder`], validates the HTTP response,
/// and deserializes the response body into the expected type `T`.
/// 
/// # Errors
///
/// Returns a [`reqwest::Error`] if:
/// - The request fails to send (e.g., network issues)
/// - The response returns a non-success status code (e.g., 400, 500)
/// - JSON deserialization into type `T` fails
async fn request<T: DeserializeOwned>(
    request_builder: RequestBuilder,
) -> Result<T, reqwest::Error> {
    let response = request_builder.send().await?.error_for_status()?;

    response.json::<T>().await
}

#[async_trait::async_trait]
impl AssetInfoProvider for RestApi {
    type Error = ProviderError;

    /// Fetches asset information for a list of coin IDs from the CoinGecko REST API.
    /// 
    /// This method retrieves current USD prices for the given `ids` by calling 
    /// [`RestApi::get_simple_price_usd`] iteratively and transforms the results into a vector of [`AssetInfo`] structs.
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
    /// [`RestApi::get_simple_price_usd`]: crate::api::RestApi::get_simple_price_usd
    /// [`AssetInfo`]: bothan_lib::types::AssetInfo
    /// [`Decimal`]: rust_decimal::Decimal
    /// [`ProviderError`]: crate::api::error::ProviderError
    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error> {
        self.get_simple_price_usd(ids)
            .await?
            .into_iter()
            .map(|(id, p)| {
                let price = Decimal::from_f64_retain(p.usd).ok_or(ProviderError::InvalidValue)?;
                Ok(AssetInfo::new(id, price, p.last_updated_at))
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use super::*;
    use crate::api::RestApiBuilder;

    async fn setup() -> (ServerGuard, RestApi) {
        let server = Server::new_async().await;

        let api = RestApiBuilder::default()
            .with_url(server.url())
            .build()
            .unwrap();

        (server, api)
    }

    trait MockCoinGecko {
        fn set_successful_coin_list(&mut self, coin_list: &[Coin]) -> Mock;
        fn set_failed_coin_list(&mut self) -> Mock;
        fn set_successful_simple_price(
            &mut self,
            ids: &[&str],
            prices: &HashMap<String, Price>,
        ) -> Mock;
        fn set_arbitrary_simple_price<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_coins_market(&mut self, ids: &[&str]) -> Mock;
    }

    impl MockCoinGecko for ServerGuard {
        fn set_successful_coin_list(&mut self, coin_list: &[Coin]) -> Mock {
            self.mock("GET", "/coins/list")
                .with_status(200)
                .with_body(serde_json::to_string(coin_list).unwrap())
                .create()
        }

        fn set_failed_coin_list(&mut self) -> Mock {
            self.mock("GET", "/coins/list").with_status(500).create()
        }

        fn set_successful_simple_price(
            &mut self,
            ids: &[&str],
            prices: &HashMap<String, Price>,
        ) -> Mock {
            self.mock("GET", "/simple/price")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("ids".into(), ids.join(",")),
                    Matcher::UrlEncoded("vs_currencies".into(), "usd".into()),
                    Matcher::UrlEncoded("include_last_updated_at".into(), "true".into()),
                    Matcher::UrlEncoded("precision".into(), "full".into()),
                ]))
                .with_status(200)
                .with_body(serde_json::to_string(prices).unwrap())
                .create()
        }

        fn set_arbitrary_simple_price<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock {
            self.mock("GET", "/simple/price")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("ids".into(), ids.join(",")),
                    Matcher::UrlEncoded("vs_currencies".into(), "usd".into()),
                    Matcher::UrlEncoded("include_last_updated_at".into(), "true".into()),
                    Matcher::UrlEncoded("precision".into(), "full".into()),
                ]))
                .with_status(200)
                .with_body(data)
                .create()
        }

        fn set_failed_coins_market(&mut self, ids: &[&str]) -> Mock {
            self.mock("GET", "/simple/price")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("ids".into(), ids.join(",")),
                    Matcher::UrlEncoded("vs_currencies".into(), "usd".into()),
                    Matcher::UrlEncoded("include_last_updated_at".into(), "true".into()),
                    Matcher::UrlEncoded("precision".into(), "full".into()),
                ]))
                .with_status(500)
                .create()
        }
    }

    #[tokio::test]
    async fn test_successful_get_coin_list() {
        let (mut server, client) = setup().await;

        let coin_list = vec![Coin {
            id: "bitcoin".to_string(),
            symbol: "btc".to_string(),
            name: "Bitcoin".to_string(),
        }];
        let mock = server.set_successful_coin_list(&coin_list);

        let result = client.get_coins_list().await;
        mock.assert();
        assert_eq!(result.unwrap(), coin_list);
    }

    #[tokio::test]
    async fn test_unsuccessful_get_coin_list() {
        let (mut server, client) = setup().await;

        let mock = server.set_failed_coin_list();

        let result = client.get_coins_list().await;
        mock.assert();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_successful_get_simple_price() {
        let (mut server, client) = setup().await;
        let prices: HashMap<String, Price> = HashMap::from([(
            "bitcoin".to_string(),
            Price {
                usd: 42000.69,
                last_updated_at: 42000,
            },
        )]);

        let mocks = server.set_successful_simple_price(&["bitcoin"], &prices);

        let result = client.get_simple_price_usd(&["bitcoin"]).await;

        mocks.assert();
        assert_eq!(result.unwrap(), prices);
    }

    #[tokio::test]
    async fn test_get_simple_price_with_missing_data() {
        let (mut server, client) = setup().await;

        let prices = HashMap::from([(
            "bitcoin".to_string(),
            Price {
                usd: 42000.69,
                last_updated_at: 42000,
            },
        )]);

        let ids = &["bitcoin", "abba"];
        let mocks = server.set_successful_simple_price(ids, &prices);

        let result = client.get_simple_price_usd(ids).await;

        mocks.assert();
        assert_eq!(result.unwrap(), prices);
    }

    #[tokio::test]
    async fn test_get_simple_price_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let ids = &["apple_pie"];
        let mock = server.set_arbitrary_simple_price(ids, "abc");

        let result = client.get_simple_price_usd(ids).await;

        mock.assert();

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_failed_get_simple_price() {
        let (mut server, client) = setup().await;
        let mock = server.set_failed_coins_market(&["bitcoin"]);

        let result = client.get_simple_price_usd(&["bitcoin"]).await;
        mock.assert();
        assert!(result.is_err());
    }
}
