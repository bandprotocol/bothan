use std::collections::HashMap;

use chrono::Utc;
use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::Error;
use crate::api::types::{Price, SymbolPrice};

pub struct CryptoCompareRestAPI {
    url: Url,
    client: Client,
}

impl CryptoCompareRestAPI {
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_multi_symbol_price(
        &self,
        ids: &[&str],
    ) -> Result<Vec<Option<SymbolPrice>>, Error> {
        let url = format!("{}data/pricemulti", self.url);
        let params = vec![("fsyms", ids.join(",")), ("tsyms", "usd".to_string())];

        let builder_with_query = self.client.get(&url).query(&params);
        let response: Response = send_request(builder_with_query).await?;
        let symbol_prices = parse_response::<HashMap<String, Price>>(response).await?;
        let now = Utc::now().timestamp() as u64;

        let results = ids
            .iter()
            .map(|id| {
                symbol_prices.get(*id).map(|price| SymbolPrice {
                    id: (*id).to_string(),
                    current_price: price.usd,
                    timestamp: now,
                })
            })
            .collect();
        Ok(results)
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

    use crate::api::CryptoCompareRestAPIBuilder;

    use super::*;

    const UNIX_EPOCH: u64 = 0; // Unix epoch starts at 0 seconds

    pub(crate) async fn setup() -> (ServerGuard, CryptoCompareRestAPI) {
        let server = Server::new_async().await;

        let mut builder = CryptoCompareRestAPIBuilder::default();
        builder.set_url(&server.url());
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) fn is_valid_timestamp(timestamp: u64) -> bool {
        // Assuming a reasonable upper bound for current time
        let current_time = chrono::Utc::now().timestamp() as u64;

        // Assuming timestamps must be within a certain range (e.g., the last 10 years)
        let min_valid_timestamp = UNIX_EPOCH;
        let max_valid_timestamp = current_time + (10 * 365 * 24 * 60 * 60); // 10 years in seconds

        timestamp >= min_valid_timestamp && timestamp <= max_valid_timestamp
    }

    pub(crate) fn assert_symbol_prices_eq(
        symbol_prices: Result<Vec<Option<SymbolPrice>>, Error>,
        expected: Result<Vec<Option<SymbolPrice>>, Error>,
    ) {
        match (symbol_prices, expected) {
            (Ok(symbol_prices), Ok(expected)) => {
                assert_eq!(symbol_prices.len(), expected.len());
                for (symbol_price, expected) in symbol_prices.iter().zip(expected.iter()) {
                    if let (Some(symbol_price), Some(expected)) = (symbol_price, expected) {
                        assert_eq!(symbol_price.id, expected.id);
                        assert_eq!(symbol_price.current_price, expected.current_price);

                        // Assert timestamp validity directly
                        assert!(is_valid_timestamp(symbol_price.timestamp));
                        assert!(is_valid_timestamp(expected.timestamp));
                    } else {
                        assert_eq!(symbol_price, expected);
                    }
                }
            }
            (Err(symbol_prices), Err(expected)) => {
                assert_eq!(symbol_prices, expected);
            }
            _ => panic!("unexpected result"),
        }
    }

    pub(crate) trait MockCryptoCompare {
        fn set_successful_multi_symbol_price(
            &mut self,
            ids: &[&str],
            symbol_prices: &[SymbolPrice],
        ) -> Mock;
        fn set_arbitrary_multi_symbol_price<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_multi_symbol_price(&mut self, ids: &[&str]) -> Mock;
    }

    impl MockCryptoCompare for ServerGuard {
        fn set_successful_multi_symbol_price(
            &mut self,
            ids: &[&str],
            symbol_prices: &[SymbolPrice],
        ) -> Mock {
            let price_map = symbol_prices
                .iter()
                .map(|symbol_price| {
                    (
                        symbol_price.id.clone(),
                        Price {
                            usd: symbol_price.current_price,
                        },
                    )
                })
                .collect::<HashMap<_, _>>();

            self.mock("GET", "/data/pricemulti")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("fsyms".into(), ids.join(",")),
                    Matcher::UrlEncoded("tsyms".into(), "usd".into()),
                ]))
                .with_status(200)
                .with_body(serde_json::to_string(&price_map).unwrap())
                .create()
        }

        fn set_arbitrary_multi_symbol_price<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock {
            self.mock("GET", "/data/pricemulti")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("fsyms".into(), ids.join(",")),
                    Matcher::UrlEncoded("tsyms".into(), "usd".into()),
                ]))
                .with_status(200)
                .with_body(data)
                .create()
        }

        fn set_failed_multi_symbol_price(&mut self, ids: &[&str]) -> Mock {
            self.mock("GET", "/data/pricemulti")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("fsyms".into(), ids.join(",")),
                    Matcher::UrlEncoded("tsyms".into(), "usd".into()),
                ]))
                .with_status(500)
                .create()
        }
    }

    #[tokio::test]
    async fn test_successful_get_coin_market() {
        let (mut server, client) = setup().await;
        let ids = &["btc", "eth"];

        let now = Utc::now().timestamp() as u64;

        let symbol_prices = vec![
            SymbolPrice {
                id: "btc".to_string(),
                current_price: 42000.69,
                timestamp: now,
            },
            SymbolPrice {
                id: "eth".to_string(),
                current_price: 2000.0,
                timestamp: now,
            },
        ];

        let mock = server.set_successful_multi_symbol_price(ids, &symbol_prices);

        let result = client.get_multi_symbol_price(ids).await;

        mock.assert();

        let expected_result = symbol_prices.into_iter().map(Some).collect();
        assert_symbol_prices_eq(result, Ok(expected_result));
    }

    #[tokio::test]
    async fn test_get_coin_market_with_missing_data() {
        let (mut server, client) = setup().await;
        let ids = &["btc", "eth"];
        let now = Utc::now().timestamp() as u64;

        let symbol_prices = vec![SymbolPrice {
            id: "btc".to_string(),
            current_price: 42000.69,
            timestamp: now,
        }];
        let mock = server.set_successful_multi_symbol_price(ids, &symbol_prices);

        let result = client.get_multi_symbol_price(ids).await;

        mock.assert();

        let expected_result: Vec<Option<SymbolPrice>> = vec![Some(symbol_prices[0].clone()), None];
        assert_symbol_prices_eq(result, Ok(expected_result));
    }

    #[tokio::test]
    async fn test_get_coin_market_with_unparseable_data() {
        let (mut server, client) = setup().await;
        let ids = &["apple_pie"];
        let mock = server.set_arbitrary_multi_symbol_price(ids, "abc");

        let result = client.get_multi_symbol_price(ids).await;

        mock.assert();

        let expected_err = Error::Reqwest(
            "error decoding response body: expected value at line 1 column 1".to_string(),
        );
        assert_symbol_prices_eq(result, Err(expected_err));
    }

    #[tokio::test]
    async fn test_failed_get_coin_market() {
        let (mut server, client) = setup().await;
        let ids = &["btc"];
        let mock = server.set_failed_multi_symbol_price(ids);

        let result = client.get_multi_symbol_price(ids).await;

        mock.assert();

        let expected_err = Error::Http(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
        assert_symbol_prices_eq(result, Err(expected_err));
    }
}
