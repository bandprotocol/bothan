use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::RestApiError;
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
    ) -> Result<Vec<Option<f64>>, RestApiError> {
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

async fn send_request(request_builder: RequestBuilder) -> Result<Response, RestApiError> {
    let response = request_builder.send().await?;

    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(RestApiError::Http(status));
    }

    Ok(response)
}

async fn parse_response<T: serde::de::DeserializeOwned>(
    response: Response,
) -> Result<T, RestApiError> {
    Ok(response.json::<T>().await?)
}
