use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::RestAPIError;
use crate::api::types::Price;

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
    ) -> Result<Vec<Option<f64>>, RestAPIError> {
        let url = format!("{}data/pricemulti", self.url);
        let params = vec![("fsyms", ids.join(",")), ("tsyms", "usd".to_string())];

        let builder_with_query = self.client.get(&url).query(&params);
        let response: Response = send_request(builder_with_query).await?;
        let symbol_prices = parse_response::<HashMap<String, Price>>(response).await?;

        let results = ids
            .iter()
            .map(|id| symbol_prices.get(*id).map(|p| p.usd))
            .collect::<Vec<Option<f64>>>();
        Ok(results)
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

async fn parse_response<T: serde::de::DeserializeOwned>(
    response: Response,
) -> Result<T, RestAPIError> {
    Ok(response.json::<T>().await?)
}

#[cfg(test)]
pub(crate) mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use crate::api::CryptoCompareRestAPIBuilder;

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, CryptoCompareRestAPI) {
        let server = Server::new_async().await;

        let mut builder = CryptoCompareRestAPIBuilder::default();
        builder.set_url(&server.url());
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) trait MockCryptoCompare {
        fn set_successful_multi_symbol_price(&mut self, ids: &[&str], prices: &[f64]) -> Mock;
        fn set_arbitrary_multi_symbol_price<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            ids: &[&str],
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_multi_symbol_price(&mut self, ids: &[&str]) -> Mock;
    }

    impl MockCryptoCompare for ServerGuard {
        fn set_successful_multi_symbol_price(&mut self, ids: &[&str], prices: &[f64]) -> Mock {
            let price_map = ids
                .iter()
                .zip(prices.iter())
                .map(|(&id, &price)| (id, Price { usd: price }))
                .collect::<HashMap<&str, Price>>();

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

        let prices = vec![42000.69, 2000.0];

        let mock = server.set_successful_multi_symbol_price(ids, &prices);

        let result = client.get_multi_symbol_price(ids).await;

        mock.assert();

        let expected_result = prices.into_iter().map(Some).collect();
        assert_eq!(result, Ok(expected_result));
    }

    #[tokio::test]
    async fn test_get_coin_market_with_missing_data() {
        let (mut server, client) = setup().await;
        let ids = &["btc", "eth"];

        let prices = vec![42000.69];

        let mock = server.set_successful_multi_symbol_price(ids, &prices);

        let result = client.get_multi_symbol_price(ids).await;

        mock.assert();

        let expected_result: Vec<Option<f64>> = vec![Some(prices[0]), None];
        assert_eq!(result, Ok(expected_result));
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
        assert_eq!(result, Err(expected_err));
    }

    #[tokio::test]
    async fn test_failed_get_coin_market() {
        let (mut server, client) = setup().await;
        let ids = &["btc"];
        let mock = server.set_failed_multi_symbol_price(ids);

        let result = client.get_multi_symbol_price(ids).await;

        mock.assert();

        let expected_err = Error::Http(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result, Err(expected_err));
    }
}
