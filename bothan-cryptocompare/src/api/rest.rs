use std::collections::HashMap;

use chrono::Utc;
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
            .filter_map(|id| {
                market_data.get(*id).map(|price| {
                    Some(Market {
                        id: (*id).to_string(),
                        current_price: price.usd,
                        timestamp: Utc::now().timestamp() as u64,
                    })
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
mod tests {
    use crate::api::CryptoCompareRestAPIBuilder;

    use super::*;

    #[tokio::test]
    async fn test_get_coins_market() {
        let url = Url::parse("https://min-api.cryptocompare.com").unwrap();
        let client = Client::new();
        let api = CryptoCompareRestAPI::new(url, client);

        let ids = vec!["BTC", "ETH"];
        let result = api.get_coins_market(&ids).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_builder() {
        let builder = CryptoCompareRestAPIBuilder::default();
        let api = builder.build().unwrap();
        let ids = vec!["BTC", "ETH"];
        let result = api.get_coins_market(&ids).await.unwrap();
        println!("{:?}", result);
    }
}
