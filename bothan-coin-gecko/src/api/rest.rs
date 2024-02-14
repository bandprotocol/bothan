use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, RequestBuilder, Response, Url};

use crate::api::error::Error;
use crate::api::types::{Coin, Market};

const MAX_PAGE_SIZE: usize = 250;
const DEFAULT_USER_AGENT: &str = "Bothan";
const DEFAULT_URL: &str = "https://api.coingecko.com/api/v3";

pub struct CoinGeckoRest {
    url: Url,
    client: Client,
}

impl CoinGeckoRest {
    pub fn new(url: &str, api_key: Option<&str>) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_str(DEFAULT_USER_AGENT)?);

        if let Some(key) = api_key {
            let mut val = HeaderValue::from_str(key)?;
            val.set_sensitive(true);
            headers.insert("x-cg-pro-api-key", val);
        }

        Ok(Self {
            url: Url::parse(url)?,
            client: ClientBuilder::new().default_headers(headers).build()?,
        })
    }

    pub fn default() -> Result<Self, Error> {
        Self::new(DEFAULT_URL, None)
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
    // let re = request_builder.header("User-Agent", "Bothan");
    let response = request_builder.send().await?;

    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        return Err(Error::Http(status));
    }

    Ok(response)
}
