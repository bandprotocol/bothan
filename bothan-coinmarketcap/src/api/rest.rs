use std::collections::HashMap;

use itertools::Itertools;
use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::RestApiError as Error;
use crate::api::types::{Quote, Response as CmcResponse};

pub struct CoinMarketCapRestAPI {
    url: Url,
    client: Client,
}

impl CoinMarketCapRestAPI {
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_latest_quotes(&self, ids: &[usize]) -> Result<Vec<Option<Quote>>, Error> {
        let url = format!("{}v2/cryptocurrency/quotes/latest", self.url);
        let ids_string = ids.iter().map(|id| id.to_string()).join(",");
        let params = vec![("id", ids_string)];

        let builder_with_query = self.client.get(&url).query(&params);
        let response = send_request(builder_with_query).await?;
        let cmc_response = response
            .json::<CmcResponse<HashMap<String, Quote>>>()
            .await?;
        let quote_map = cmc_response.data;

        let quotes = ids
            .iter()
            .map(|id| quote_map.get(&id.to_string()).cloned())
            .collect();
        Ok(quotes)
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

#[cfg(test)]
pub(crate) mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use crate::api::types::{PriceQuote, PriceQuotes, Status};
    use crate::api::CoinMarketCapRestAPIBuilder;

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, CoinMarketCapRestAPI) {
        let server = Server::new_async().await;

        let mut builder = CoinMarketCapRestAPIBuilder::default();
        builder.set_url(&server.url());
        builder.set_api_key("test");
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
                    market_cap: 10000000.0,
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
        let expected_result = quotes.into_iter().map(Some).collect();
        mock.assert();
        assert_eq!(result, Ok(expected_result));
    }

    #[tokio::test]
    async fn test_successful_get_quotes_with_missing_data() {
        let (mut server, client) = setup().await;

        let quotes = vec![mock_quote()];

        let mock = server.set_successful_quotes(&["1", "0"], &quotes);

        let result = client.get_latest_quotes(&[1, 0]).await;

        mock.assert();
        let expected_result = vec![Some(quotes[0].clone()), None];
        assert_eq!(result, Ok(expected_result));
    }

    #[tokio::test]
    async fn test_get_quotes_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let mock = server.set_arbitrary_quotes(&["1"], "abc");

        let result = client.get_latest_quotes(&[1]).await;

        mock.assert();

        let expected_err = Error::Reqwest(
            "error decoding response body: expected value at line 1 column 1".to_string(),
        );
        assert_eq!(result, Err(expected_err));
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
