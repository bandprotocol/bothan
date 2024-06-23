use reqwest::{Client, RequestBuilder, Response as ReqwestResponse, Url};

use crate::api::error::RestAPIError;
use crate::api::types::{Response, Ticker};

/// A client for interacting with the HTX REST API.
pub struct HtxRestAPI {
    url: Url,
    client: Client,
}

impl HtxRestAPI {
    /// Creates a new instance of `HtxRestAPI`.
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    /// Retrieves the latest tickers from the HTX API.
    pub async fn get_latest_tickers(&self) -> Result<Response<Vec<Ticker>>, RestAPIError> {
        let url = format!("{}market/tickers", self.url);

        let request = self.client.get(&url);
        let response = send_request(request).await?;
        let api_response = response.json::<Response<Vec<Ticker>>>().await?;
        Ok(api_response)
    }
}

/// Sends an HTTP request and checks for HTTP errors.
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
    use mockito::{Mock, Server, ServerGuard};

    use crate::api::types::Status;
    use crate::api::HtxRestAPIBuilder;

    use super::*;

    pub(crate) async fn setup() -> (ServerGuard, HtxRestAPI) {
        let server = Server::new_async().await;

        let mut builder = HtxRestAPIBuilder::default();
        builder.with_url(&server.url());
        let api = builder.build().unwrap();

        (server, api)
    }

    pub(crate) fn mock_ticker() -> Ticker {
        Ticker {
            symbol: "btcusdt".to_string(),
            open: 79000.0,
            high: 81000.0,
            low: 78000.0,
            close: 80000.0,
            amount: 100.0,
            vol: 100000.0,
            count: 1000,
            bid: 80000.0,
            bid_size: 100.0,
            ask: 80000.0,
            ask_size: 100.0,
        }
    }

    pub(crate) trait MockHtx {
        fn set_successful_tickers(&mut self, quotes: &[Ticker], timestamp: usize) -> Mock;
        fn set_arbitrary_tickers<StrOrBytes: AsRef<[u8]>>(&mut self, data: StrOrBytes) -> Mock;
        fn set_failed_tickers(&mut self) -> Mock;
    }

    impl MockHtx for ServerGuard {
        fn set_successful_tickers(&mut self, ticker: &[Ticker], timestamp: usize) -> Mock {
            let response = Response::<Vec<Ticker>> {
                data: ticker.to_vec(),
                status: Status::Ok,
                timestamp,
            };
            self.mock("GET", "/market/tickers")
                .with_status(200)
                .with_body(serde_json::to_string(&response).unwrap())
                .create()
        }

        fn set_arbitrary_tickers<StrOrBytes: AsRef<[u8]>>(&mut self, data: StrOrBytes) -> Mock {
            self.mock("GET", "/market/tickers")
                .with_status(200)
                .with_body(data)
                .create()
        }

        fn set_failed_tickers(&mut self) -> Mock {
            self.mock("GET", "/market/tickers")
                .with_status(500)
                .create()
        }
    }

    #[tokio::test]
    async fn test_successful_get_quotes() {
        let (mut server, client) = setup().await;

        let quotes = vec![mock_ticker()];
        let mock = server.set_successful_tickers(&quotes, 10000);

        let result = client.get_latest_tickers().await;
        mock.assert();
        assert_eq!(result.unwrap().data, quotes);
    }

    #[tokio::test]
    async fn test_get_quotes_with_unparseable_data() {
        let (mut server, client) = setup().await;

        let mock = server.set_arbitrary_tickers("abc");

        let result = client.get_latest_tickers().await;

        mock.assert();
        let expected_err = RestAPIError::Reqwest("error decoding response body".to_string());
        assert_eq!(result, Err(expected_err));
    }

    #[tokio::test]
    async fn test_failed_get_quotes() {
        let (mut server, client) = setup().await;
        let mock = server.set_failed_tickers();

        let result = client.get_latest_tickers().await;
        mock.assert();
        assert!(result.is_err());
    }
}
