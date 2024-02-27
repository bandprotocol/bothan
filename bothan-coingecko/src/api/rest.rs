use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::Error;
use crate::api::types::{Coin, Market};

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

    pub async fn get_coins_market(
        &self,
        ids: &[&str],
        page_size: usize,
        page: usize,
    ) -> Result<Vec<Option<Market>>, Error> {
        let url = format!("{}coins/markets", self.url);
        let params = vec![
            ("vs_currency", "usd".to_string()),
            ("per_page", page_size.to_string()),
            ("ids", ids.join(",")),
            ("page", page.to_string()),
        ];

        let builder_with_query = self.client.get(&url).query(&params);
        let response = send_request(builder_with_query).await?;
        let market_data = parse_response::<Vec<Market>>(response).await?;
        let mut market_data_map: HashMap<String, Market> =
            HashMap::from_iter(market_data.into_iter().map(|m| (m.id.clone(), m)));
        Ok(ids
            .iter()
            .map(|id| market_data_map.remove(&id.to_string()))
            .collect())
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
