use reqwest::{Client, RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;

use crate::api::error::RestAPIError;
use crate::api::types::{Coin, Market, Order, DEFAULT_ORDER, DEFAULT_PAGE, DEFAULT_PER_PAGE};

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
    pub async fn get_coins_list(&self) -> Result<Vec<Coin>, RestAPIError> {
        let url = format!("{}coins/list", self.url);
        let builder = self.client.get(url);
        let response = send_request(builder).await?;

        Ok(response.json::<Vec<Coin>>().await?)
    }

    /// Retrieves market data for the specified coins from the CoinGecko API.
    pub async fn get_coins_market<T: AsRef<str>>(
        &self,
        ids: &[T],
        order: Option<Order>,
        page_size: Option<usize>,
        page: Option<usize>,
    ) -> Result<Vec<Market>, RestAPIError> {
        let url = format!("{}coins/markets", self.url);
        let ids = ids.iter().map(|id| id.as_ref()).collect::<Vec<&str>>();

        let order = order.unwrap_or(DEFAULT_ORDER).to_string();
        let page_size = page_size.unwrap_or(DEFAULT_PER_PAGE).to_string();
        let page = page.unwrap_or(DEFAULT_PAGE).to_string();

        let params = vec![
            ("vs_currency", "usd".to_string()),
            ("ids", ids.join(",")),
            ("order", order),
            ("per_page", page_size),
            ("page", page),
        ];

        let builder_with_query = self.client.get(&url).query(&params);
        let response = send_request(builder_with_query).await?;
        let markets = parse_response::<Vec<Market>>(response).await?;
        Ok(markets)
    }
}

async fn send_request(request_builder: RequestBuilder) -> Result<Response, RestAPIError> {
    let response = request_builder.send().await?;

    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(RestAPIError::Http(status));
    }

    Ok(response)
}

async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, RestAPIError> {
    Ok(response.json::<T>().await?)
}

#[cfg(test)]
pub(crate) mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use crate::api::CoinGeckoRestAPIBuilder;

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, CoinGeckoRestAPI) {
        let server = Server::new_async().await;

        let mut builder = CoinGeckoRestAPIBuilder::default();
        builder.with_url(&server.url());
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) trait MockCoinGecko {
        fn set_successful_coin_list(&mut self, coin_list: &[Coin]) -> Mock;
        fn set_failed_coin_list(&mut self) -> Mock;
        fn set_successful_coins_market(&mut self, ids: &[&str], market: &[Market]) -> Vec<Mock>;
        fn set_arbitrary_coins_market<StrOrBytes: AsRef<[u8]>>(
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

        fn set_successful_coins_market(
            &mut self,
            ids: &[&str],
            coins_market: &[Market],
        ) -> Vec<Mock> {
            let page_results = coins_market
                .chunks(250)
                .enumerate()
                .collect::<Vec<(usize, &[Market])>>();

            let joined_id = ids.join(",");

            page_results
                .into_iter()
                .map(|(page, result)| {
                    self.mock("GET", "/coins/markets")
                        .match_query(Matcher::AllOf(vec![
                            Matcher::UrlEncoded("ids".into(), joined_id.clone()),
                            Matcher::UrlEncoded("vs_currency".into(), "usd".into()),
                            Matcher::UrlEncoded("per_page".into(), 250.to_string()),
                            Matcher::UrlEncoded("page".into(), (page + 1).to_string()),
                        ]))
                        .with_status(200)
                        .with_body(serde_json::to_string(result).unwrap())
                        .create()
                })
                .collect()
        }

        fn set_arbitrary_coins_market<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock {
            self.mock("GET", "/coins/markets")
                .match_query(Matcher::UrlEncoded("ids".into(), ids.join(",")))
                .with_status(200)
                .with_body(data)
                .create()
        }

        fn set_failed_coins_market(&mut self, ids: &[&str]) -> Mock {
            self.mock("GET", "/coins/markets")
                .match_query(Matcher::UrlEncoded("ids".into(), ids.join(",")))
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
    async fn test_successful_get_coin_market() {
        let (mut server, client) = setup().await;

        let coin_markets = vec![Market {
            id: "bitcoin".to_string(),
            symbol: "btc".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 42000.69,
            last_updated: "1970-01-01T00:00:00.000Z".to_string(),
        }];
        let mocks = server.set_successful_coins_market(&["bitcoin"], &coin_markets);

        let result = client
            .get_coins_market(&["bitcoin"], None, Some(250), Some(1))
            .await;
        mocks.iter().for_each(|m| m.assert());
        assert_eq!(result, Ok(coin_markets));
    }

    #[tokio::test]
    async fn test_get_coin_market_with_missing_data() {
        let (mut server, client) = setup().await;

        let coin_markets = vec![Market {
            id: "bitcoin".to_string(),
            symbol: "btc".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 42000.69,
            last_updated: "1970-01-01T00:00:00.000Z".to_string(),
        }];
        let ids = &["bitcoin", "abba"];
        let mocks = server.set_successful_coins_market(ids, &coin_markets);

        let result = client.get_coins_market(ids, None, Some(250), Some(1)).await;

        mocks.iter().for_each(|m| m.assert());
        assert_eq!(result, Ok(coin_markets));
    }

    #[tokio::test]
    async fn test_get_coin_market_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let ids = &["apple_pie"];
        let mock = server.set_arbitrary_coins_market(ids, "abc");

        let result = client.get_coins_market(ids, None, Some(250), Some(1)).await;

        mock.assert();

        let expected_err = RestAPIError::Reqwest("error decoding response body".to_string());
        assert_eq!(result, Err(expected_err));
    }

    #[tokio::test]
    async fn test_failed_get_coin_market() {
        let (mut server, client) = setup().await;
        let mock = server.set_failed_coins_market(&["bitcoin"]);

        let result = client
            .get_coins_market(&["bitcoin"], None, Some(250), Some(1))
            .await;
        mock.assert();
        assert!(result.is_err());
    }
}
