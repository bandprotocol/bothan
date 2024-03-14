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
        let results = ids
            .iter()
            .map(|id| {
                symbol_prices.get(*id).map(|price| SymbolPrice {
                    id: (*id).to_string(),
                    current_price: price.usd,
                    timestamp: Utc::now().timestamp() as u64,
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
