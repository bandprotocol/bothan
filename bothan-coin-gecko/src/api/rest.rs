use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::Error;
use crate::api::types::{Coin, Market, MAX_PAGE_SIZE};

pub struct CoinGeckoRestAPI {
    url: Url,
    client: Client,
}

impl CoinGeckoRestAPI {
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_coins_list(&self) -> Result<Vec<Coin>, Error> {
        let url = format!("{}coins/list", self.url);
        let builder = self.client.get(url);
        let response = send_request(builder).await?;

        Ok(response.json::<Vec<Coin>>().await?)
    }

    pub async fn get_coins_market(&self, ids: &[&str]) -> Vec<Result<Market, Error>> {
        let ids_per_pages = ids
            .chunks(MAX_PAGE_SIZE)
            .enumerate()
            .collect::<Vec<(usize, &[&str])>>();

        let url = format!("{}coins/markets", self.url);
        let base_params = vec![
            ("vs_currency", "usd".to_string()),
            ("per_page", MAX_PAGE_SIZE.to_string()),
            ("ids", ids.join(",")),
        ];

        let mut markets = Vec::with_capacity(ids.len());
        for (page, page_ids) in ids_per_pages {
            let mut params = base_params.clone();
            params.push(("page", (page + 1).to_string()));

            let builder_with_query = self.client.get(&url).query(&params);
            let market_data = match send_request(builder_with_query).await {
                Ok(response) => match parse_response::<Vec<Market>>(response).await {
                    Ok(markets) => {
                        let map: HashMap<String, Market> =
                            HashMap::from_iter(markets.into_iter().map(|m| (m.id.clone(), m)));
                        page_ids
                            .iter()
                            .map(|id| {
                                let val = map.get(*id).cloned();
                                if let Some(market) = val {
                                    Ok(market)
                                } else {
                                    Err(Error::InvalidID)
                                }
                            })
                            .collect::<Vec<Result<Market, Error>>>()
                    }
                    Err(e) => vec![Err(e.clone()); page_ids.len()],
                },
                Err(e) => vec![Err(e.clone()); page_ids.len()],
            };
            markets.extend(market_data);
        }
        markets
    }
}

async fn send_request(request_builder: RequestBuilder) -> Result<Response, Error> {
    let response = request_builder.send().await?;

    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(Error::Http(status));
    }

    Ok(response)
}

async fn parse_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, Error> {
    Ok(response.json::<T>().await?)
}

#[cfg(test)]
pub(crate) mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use crate::api::CoinGeckoRestAPIBuilder;

    use super::*;

    pub(crate) fn setup() -> (ServerGuard, CoinGeckoRestAPI) {
        let server = Server::new();

        let mut builder = CoinGeckoRestAPIBuilder::default();
        builder.set_url(&server.url());
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) trait MockGecko {
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

    impl MockGecko for ServerGuard {
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
                .chunks(MAX_PAGE_SIZE)
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
                            Matcher::UrlEncoded("per_page".into(), MAX_PAGE_SIZE.to_string()),
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
        let (mut server, client) = setup();

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
        let (mut server, client) = setup();

        let mock = server.set_failed_coin_list();

        let result = client.get_coins_list().await;
        mock.assert();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_successful_get_coin_market() {
        let (mut server, client) = setup();

        let coin_markets = vec![Market {
            id: "bitcoin".to_string(),
            symbol: "btc".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 42000.69,
            last_updated: "1970-01-01T00:00:00.000Z".to_string(),
        }];
        let mocks = server.set_successful_coins_market(&["bitcoin"], &coin_markets);

        let result: Result<Vec<Market>, Error> = client
            .get_coins_market(&["bitcoin"])
            .await
            .into_iter()
            .collect();
        mocks[0].assert();
        assert_eq!(result.unwrap(), coin_markets);
    }

    #[tokio::test]
    async fn test_successful_get_coin_market_with_multiple_pages() {
        let (mut server, client) = setup();

        let coin_markets = (0..251)
            .map(|i| Market {
                id: format!("{}", i),
                symbol: format!("C{}", i),
                name: format!("Coin {}", i),
                current_price: 42000.69 + i as f64,
                last_updated: "1970-01-01T00:00:00.000Z".to_string(),
            })
            .collect::<Vec<Market>>();

        let ids = coin_markets
            .iter()
            .map(|m| m.id.as_str())
            .collect::<Vec<&str>>();

        let mocks = server.set_successful_coins_market(&ids, &coin_markets);

        let results = client
            .get_coins_market(&ids)
            .await
            .into_iter()
            .collect::<Result<Vec<Market>, Error>>();
        mocks.iter().for_each(|m| m.assert());
        assert_eq!(results.unwrap(), coin_markets);
    }

    #[tokio::test]
    async fn test_get_coin_market_with_missing_data() {
        let (mut server, client) = setup();

        let coin_markets = vec![Market {
            id: "bitcoin".to_string(),
            symbol: "btc".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 42000.69,
            last_updated: "1970-01-01T00:00:00.000Z".to_string(),
        }];
        let ids = &["bitcoin", "abba"];
        let mocks = server.set_successful_coins_market(ids, &coin_markets);

        let result = client.get_coins_market(ids).await;

        mocks.iter().for_each(|m| m.assert());
        let expected_result = vec![Ok(coin_markets[0].clone()), Err(Error::InvalidID)];
        assert_eq!(result, expected_result);
    }

    #[tokio::test]
    async fn test_get_coin_market_with_unparseable_data() {
        let (mut server, client) = setup();

        let ids = &["apple_pie"];
        let mock = server.set_arbitrary_coins_market(ids, "abc");

        let result = client.get_coins_market(ids).await;

        mock.assert();

        let expected_err = Err(Error::Reqwest(
            "error decoding response body: expected value at line 1 column 1".to_string(),
        ));
        assert_eq!(result[0], expected_err);
    }

    #[tokio::test]
    async fn test_failed_get_coin_market() {
        let (mut server, client) = setup();
        let mock = server.set_failed_coins_market(&["bitcoin"]);

        let result = client.get_coins_market(&["bitcoin"]).await;
        mock.assert();
        assert!(result.iter().all(|r| r.is_err()));
    }
}
