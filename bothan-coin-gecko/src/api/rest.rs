use std::collections::HashMap;

use reqwest::{Client, RequestBuilder, Response, Url};

use crate::api::error::Error;
use crate::api::types::{Coin, Market, MAX_PAGE_SIZE};

pub struct CoinGeckoRestAPI {
    url: Url,
    client: Client,
}

impl CoinGeckoRestAPI {
    pub fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub async fn get_coins_list(&self) -> Result<Vec<Coin>, Error> {
        let url = format!("{}/coins/list", self.url);
        let builder = self.client.get(url);
        let response = send_request(builder).await?;

        Ok(response.json::<Vec<Coin>>().await?)
    }

    pub async fn get_coins_market(&self, ids: &[&str]) -> Vec<Result<Market, Error>> {
        let ids_jobs = ids
            .chunks(MAX_PAGE_SIZE)
            .enumerate()
            .collect::<Vec<(usize, &[&str])>>();

        let url = format!("{}/coins/markets", self.url);
        let base_params = vec![
            ("vs_currency", "usd".to_string()),
            ("per_page", MAX_PAGE_SIZE.to_string()),
        ];

        let mut markets = Vec::with_capacity(ids.len());
        for (page, ids) in ids_jobs {
            let mut params = base_params.clone();
            params.push(("page", (page + 1).to_string()));
            params.push(("ids", ids.join(",")));

            let builder_with_query = self.client.get(&url).query(&params);
            let market_data = match send_request(builder_with_query).await {
                Ok(response) => {
                    if let Ok(markets) = parse_response::<Vec<Market>>(response).await {
                        let map: HashMap<String, Market> =
                            HashMap::from_iter(markets.into_iter().map(|m| (m.id.clone(), m)));
                        // Not found error
                        ids.iter()
                            .map(|id| {
                                let val = map.get(*id).cloned();
                                if let Some(market) = val {
                                    Ok(market)
                                } else {
                                    Err(Error::CatchAll)
                                }
                            })
                            .collect::<Vec<Result<Market, Error>>>()
                    } else {
                        ids.iter().map(|_| Err(Error::CatchAll)).collect()
                    }
                }
                Err(_) => ids.iter().map(|_| Err(Error::CatchAll)).collect(),
            };
            markets.extend(market_data);
        }
        markets
    }
}

async fn parse_response<T>(response: Response) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    Ok(response.json::<T>().await?)
}

async fn send_request(request_builder: RequestBuilder) -> Result<Response, Error> {
    let response = request_builder.send().await?;

    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(Error::Http(status));
    }

    Ok(response)
}
