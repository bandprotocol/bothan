use reqwest::{Client, RequestBuilder, Response as ReqwestResponse, Url};

use crate::api::error::RestAPIError;
use crate::api::types::ticker::{Category, TickersResponse};
use crate::api::types::Response;

pub struct BybitRestAPI {
    url: Url,
    client: Client,
}

impl BybitRestAPI {
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_tickers(
        &self,
        category: Category,
        symbol: Option<&str>,
    ) -> Result<Response<TickersResponse>, RestAPIError> {
        let url = format!("{}v5/market/tickers", self.url);
        let mut params = vec![];
        match category {
            Category::Spot => params.push(("category", category.to_string())),
            _ => return Err(RestAPIError::UnsupportedCategory),
        };

        if let Some(sym) = symbol {
            params.push(("symbol", sym.to_string()));
        };

        let response = send_request(self.client.get(&url).query(&params)).await?;
        Ok(response.json::<Response<TickersResponse>>().await?)
    }
}

async fn send_request(request_builder: RequestBuilder) -> Result<ReqwestResponse, RestAPIError> {
    let response = request_builder.send().await?;

    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(RestAPIError::Http(status));
    }

    Ok(response)
}

#[cfg(test)]
pub(crate) mod test {
    use mockito::{Matcher, Mock, Server, ServerGuard};

    use crate::api::builder::BybitRestAPIBuilder;
    use crate::api::types::ticker::{SpotTicker, Tickers, TickersResponse};

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, BybitRestAPI) {
        let server = Server::new_async().await;

        let mut builder = BybitRestAPIBuilder::default();
        builder.with_url(&server.url());
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) fn mock_tickers_response() -> TickersResponse {
        let spot_ticker = SpotTicker {
            symbol: "BTCUSDT".to_string(),
            bid1_price: "1322".to_string(),
            bid1_size: "132211".to_string(),
            ask1_price: "1323".to_string(),
            ask1_size: "142222".to_string(),
            last_price: "1322.5".to_string(),
            prev_price_24h: "1222.1".to_string(),
            price_24h_pcnt: "11".to_string(),
            high_price_24h: "1500".to_string(),
            low_price_24h: "1100".to_string(),
            turnover_24h: "11233333".to_string(),
            volume_24h: "50000".to_string(),
        };
        TickersResponse {
            category: Some(Category::Spot),
            list: Some(Tickers::Spot(vec![spot_ticker])),
        }
    }

    pub(crate) trait MockBybit {
        fn set_successful_tickers(&mut self, quotes: &TickersResponse, timestamp: usize) -> Mock;
        fn set_arbitrary_tickers<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            category: Category,
            data: StrOrBytes,
        ) -> Mock;
        fn set_failed_tickers(&mut self, category: Category) -> Mock;
    }

    impl MockBybit for ServerGuard {
        fn set_successful_tickers(
            &mut self,
            tickers_response: &TickersResponse,
            timestamp: usize,
        ) -> Mock {
            let response = Response::<TickersResponse> {
                ret_code: 0,
                ret_msg: "OK".to_string(),
                result: tickers_response.clone(),
                time: timestamp,
            };
            let category = tickers_response.category.clone().unwrap();
            self.mock("GET", "/v5/market/tickers")
                .with_status(200)
                .with_body(serde_json::to_string(&response).unwrap())
                .match_query(Matcher::UrlEncoded("category".into(), category.to_string()))
                .create()
        }

        fn set_arbitrary_tickers<StrOrBytes: AsRef<[u8]>>(
            &mut self,
            category: Category,
            data: StrOrBytes,
        ) -> Mock {
            self.mock("GET", "/v5/market/tickers")
                .with_status(200)
                .with_body(data)
                .match_query(Matcher::UrlEncoded("category".into(), category.to_string()))
                .create()
        }

        fn set_failed_tickers(&mut self, category: Category) -> Mock {
            self.mock("GET", "/v5/market/tickers")
                .with_status(500)
                .match_query(Matcher::UrlEncoded("category".into(), category.to_string()))
                .create()
        }
    }

    #[tokio::test]
    async fn test_successful_get_quotes() {
        let (mut server, client) = setup().await;

        let mock_tickers = mock_tickers_response();
        let mock = server.set_successful_tickers(&mock_tickers, 10000);

        let result = client.get_tickers(Category::Spot, None).await;
        mock.assert();
        assert_eq!(result.unwrap().result, mock_tickers);
    }

    #[tokio::test]
    async fn test_get_quotes_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let mock = server.set_arbitrary_tickers(Category::Spot, "abc");

        let result = client.get_tickers(Category::Spot, None).await;

        mock.assert();
        let expected_err = RestAPIError::Reqwest("error decoding response body".to_string());
        assert_eq!(result, Err(expected_err));
    }

    #[tokio::test]
    async fn test_get_quote_with_unsupported_types() {
        let (_, client) = setup().await;
        let result = client.get_tickers(Category::Option, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_failed_get_quotes() {
        let (mut server, client) = setup().await;
        let mock = server.set_failed_tickers(Category::Spot);

        let result = client.get_tickers(Category::Spot, None).await;
        mock.assert();
        assert!(result.is_err());
    }
}
