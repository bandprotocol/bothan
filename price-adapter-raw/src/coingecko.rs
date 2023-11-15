use std::collections::HashMap;

use chrono::{NaiveDateTime, TimeZone, Utc};
use reqwest::{Client, Response, StatusCode};
use serde::Deserialize;

use crate::{error::Error, types::PriceInfo};

const PUBLIC_ENDPOINT: &str = "https://api.coingecko.com/api/v3/";
const PRO_ENDPOINT: &str = "https://pro-api.coingecko.com/api/v3/";

#[derive(Deserialize, Debug, Clone)]
pub struct CoinMarketResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub last_updated: String,
}

/// Base object to query CoinGecko api.
#[derive(Default)]
pub struct CoinGecko {
    api_key: String,
    url: String,
    client: Client,
}

impl CoinGecko {
    /// initiate new api object.
    pub fn new() -> Self {
        Self {
            url: PUBLIC_ENDPOINT.into(),
            ..Self::default()
        }
    }

    pub fn new_with_api_key(api_key: String) -> Self {
        Self {
            api_key,
            url: PRO_ENDPOINT.into(),
            ..Default::default()
        }
    }

    pub async fn get_prices(&self, ids: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        match self._get_prices(ids).await {
            Ok(results) => results,
            Err(err) => {
                tracing::error!("get prices error: {}", err);
                ids.iter()
                    .map(|_| Err(Error::GeneralQueryPriceError()))
                    .collect()
            }
        }
    }

    async fn _get_prices(&self, ids: &[&str]) -> Result<Vec<Result<PriceInfo, Error>>, Error> {
        let response = self.send_request(ids).await?;

        let parsed_response = response.json::<Vec<CoinMarketResponse>>().await?;

        let id_to_response = parsed_response
            .into_iter()
            .map(|item| (item.id.to_string(), item))
            .collect::<HashMap<String, CoinMarketResponse>>();

        let results = ids
            .iter()
            .map(|&id| {
                let Some(price_info) = id_to_response.get(id) else {
                    return Err(Error::NotFound("price info".into()));
                };

                let naive_datetime = match NaiveDateTime::parse_from_str(
                    &price_info.last_updated,
                    "%Y-%m-%dT%H:%M:%S.%fZ",
                ) {
                    Ok(datetime) => datetime,
                    Err(err) => return Err(err.into()),
                };

                let datetime = Utc.from_utc_datetime(&naive_datetime);

                Ok(PriceInfo {
                    id: id.to_string(),
                    price: price_info.current_price,
                    timestamp: datetime.timestamp() as u64,
                })
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    async fn send_request(&self, unique_ids: &[&str]) -> Result<Response, Error> {
        let mut query = vec![("ids", unique_ids.join(",")), ("vs_currency", "usd".into())];
        if !self.api_key.is_empty() {
            query.push(("x_cg_pro_api_key", self.api_key.clone()));
        }

        let url = format!("{}/coins/markets", self.url);
        let response = self.client.get(url).query(&query).send().await?;
        let response_status = response.status();
        if response_status != StatusCode::OK {
            tracing::error!(
                "query request get error status {} {}",
                response_status,
                response.text().await?
            );
            return Err(Error::ResponseStatusNotOk(response_status));
        }

        Ok(response)
    }
}
