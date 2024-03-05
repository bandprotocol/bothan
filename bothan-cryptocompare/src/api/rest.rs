use std::collections::HashMap;

#[cfg(not(test))]
use chrono::Utc;

#[cfg(test)]
use crate::mock::mock_utc as Utc;

use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::Error;
use crate::api::types::{Market, Price};

pub struct CryptoCompareRestAPI {
    url: Url,
    client: Client,
}

impl CryptoCompareRestAPI {
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_coins_market(&self, ids: &[&str]) -> Result<Vec<Option<Market>>, Error> {
        let url = format!("{}data/pricemulti", self.url);
        let params = vec![("fsyms", ids.join(",")), ("tsyms", "usd".to_string())];

        let builder_with_query = self.client.get(&url).query(&params);
        let response: Response = send_request(builder_with_query).await?;
        let market_data = parse_response::<HashMap<String, Price>>(response).await?;
        Ok(ids
            .iter()
            .map(|id| {
                market_data.get(*id).map(|price| Market {
                    id: (*id).to_string(),
                    current_price: price.usd,
                    timestamp: Utc::now().timestamp() as u64,
                })
            })
            .collect())
    }
}

async fn send_request(request_builder: RequestBuilder) -> Result<Response, Error> {
    let response: Response = request_builder.send().await?;

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
    use crate::mock::mock_utc;

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, CryptoCompareRestAPI) {
        let server = Server::new_async().await;

        let mut builder = CryptoCompareRestAPIBuilder::default();
        builder.set_url(&server.url());
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) trait MockCryptoCompare {
        fn set_successful_coins_market(&mut self, ids: &[&str], coins_market: &[Market]) -> Mock;
        fn set_arbitrary_coins_market<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_coins_market(&mut self, ids: &[&str]) -> Mock;
    }

    impl MockCryptoCompare for ServerGuard {
        fn set_successful_coins_market(&mut self, ids: &[&str], coins_market: &[Market]) -> Mock {
            let price_map = coins_market
                .iter()
                .map(|market| {
                    (
                        market.id.clone(),
                        Price {
                            usd: market.current_price,
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

        fn set_arbitrary_coins_market<StrOrBytes: AsRef<[u8]>>(
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

        fn set_failed_coins_market(&mut self, ids: &[&str]) -> Mock {
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
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let (mut server, client) = setup().await;
        let ids = &["btc", "eth"];
        let coin_markets = vec![
            Market {
                id: "btc".to_string(),
                current_price: 42000.69,
                timestamp: now,
            },
            Market {
                id: "eth".to_string(),
                current_price: 2000.0,
                timestamp: now,
            },
        ];
        let mock = server.set_successful_coins_market(ids, &coin_markets);

        let result = client.get_coins_market(ids).await;

        mock.assert();

        let expected_result = coin_markets.into_iter().map(Some).collect();
        assert_eq!(result, Ok(expected_result));
    }

    #[tokio::test]
    async fn test_get_coin_market_with_missing_data() {
        // Set the timestamp to a fixed value for testing
        let timestamp_millis = 1694615225000;
        mock_utc::set_timestamp_millis(timestamp_millis);

        let now = mock_utc::now().timestamp() as u64;

        let (mut server, client) = setup().await;
        let ids = &["btc", "eth"];
        let coin_markets = vec![Market {
            id: "btc".to_string(),
            current_price: 42000.69,
            timestamp: now,
        }];
        let mock = server.set_successful_coins_market(ids, &coin_markets);

        let result = client.get_coins_market(ids).await;

        mock.assert();

        let expected_result: Vec<Option<Market>> = vec![Some(coin_markets[0].clone()), None];
        assert_eq!(result, Ok(expected_result));
    }

    #[tokio::test]
    async fn test_get_coin_market_with_unparseable_data() {
        let (mut server, client) = setup().await;
        let ids = &["apple_pie"];
        let mock = server.set_arbitrary_coins_market(ids, "abc");

        let result = client.get_coins_market(ids).await;

        mock.assert();

        let expected_err = Error::Reqwest(
            "error decoding response body: expected value at line 1 column 1".to_string(),
        );
        assert_eq!(result, Err(expected_err));
    }

    #[tokio::test]
    async fn test_failed_get_coin_market() {
        let (mut server, client) = setup().await;
        let ids = &["btc"];
        let mock = server.set_failed_coins_market(ids);

        let result = client.get_coins_market(ids).await;

        mock.assert();

        let expected_err = Error::Http(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result, Err(expected_err));
    }
}
