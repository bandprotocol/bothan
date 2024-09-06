use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;

use crate::api::error::SendError;
use crate::api::types::{Coin, Price};

/// A client for interacting with the CoinGecko REST API.
pub struct CoinGeckoRestAPI {
    url: Url,
    client: Client,
}

impl CoinGeckoRestAPI {
    /// Creates a new instance of `CoinGeckoRestAPI`.
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    /// Retrieves a list of coins from the CoinGecko API.
    pub async fn get_coins_list(&self) -> Result<Vec<Coin>, SendError> {
        let url = format!("{}coins/list", self.url);
        let builder = self.client.get(url);

        request::<Vec<Coin>>(builder).await
    }

    /// Retrieves market data for the specified coins from the CoinGecko API.
    pub async fn get_simple_price_usd<T: AsRef<str>>(
        &self,
        ids: &[T],
    ) -> Result<HashMap<String, Price>, SendError> {
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

async fn request<T: DeserializeOwned>(request_builder: RequestBuilder) -> Result<T, SendError> {
    let response = request_builder
        .send()
        .await
        .map_err(|e| SendError::FailedRequest(e.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        return Err(SendError::UnsuccessfulResponse(status));
    }

    response
        .json::<T>()
        .await
        .map_err(|e| SendError::ParseResponseFailed(e.to_string()))
}

#[cfg(test)]
pub(crate) mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use crate::api::CoinGeckoRestAPIBuilder;

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, CoinGeckoRestAPI) {
        let server = Server::new_async().await;

        let api = CoinGeckoRestAPIBuilder::default()
            .with_url(server.url())
            .build()
            .unwrap();

        (server, api)
    }

    pub(crate) trait MockCoinGecko {
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
        let prices = HashMap::from([(
            "bitcoin".to_string(),
            Price {
                usd: 42000.69,
                last_updated_at: 42000,
            },
        )]);

        let mocks = server.set_successful_simple_price(&["bitcoin"], &prices);

        let result = client.get_simple_price_usd(&["bitcoin"]).await;

        mocks.assert();
        assert_eq!(result, Ok(prices));
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
        assert_eq!(result, Ok(prices));
    }

    #[tokio::test]
    async fn test_get_simple_price_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let ids = &["apple_pie"];
        let mock = server.set_arbitrary_simple_price(ids, "abc");

        let result = client.get_simple_price_usd(ids).await;

        mock.assert();

        let expected_err =
            SendError::ParseResponseFailed("error decoding response body".to_string());
        assert_eq!(result, Err(expected_err));
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
