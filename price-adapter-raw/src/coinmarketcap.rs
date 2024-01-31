use std::{collections::HashMap, vec};

use chrono::{NaiveDateTime, TimeZone, Utc};
use reqwest::{Client, Response, StatusCode};
use serde::Deserialize;

use crate::{error::Error, types::PriceInfo};

const PRO_ENDPOINT: &str = "https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest";

#[derive(Deserialize, Debug, Clone)]
pub struct Currency {
    price: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Quote {
    #[serde(rename = "USD")]
    usd: Currency,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoinInfo {
    pub slug: String,
    pub last_updated: String,
    pub quote: Quote,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoinMarketResponse {
    data: HashMap<String, CoinInfo>,
}

/// Base object to query CoinMarketCap api.
#[derive(Default)]
pub struct CoinMarketCap {
    api_key: String,
    url: String,
    client: Client,
}

impl CoinMarketCap {
    /// initiate new api object.
    pub fn new_with_api_key(api_key: String) -> Self {
        Self {
            api_key,
            url: PRO_ENDPOINT.into(),
            ..Self::default()
        }
    }

    pub async fn get_prices(&self, ids: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        match self._get_prices(ids).await {
            Ok(results) => results,
            Err(err) => {
                tracing::trace!("get prices error: {}", err);
                ids.iter()
                    .map(|_| Err(Error::GeneralQueryPriceError()))
                    .collect()
            }
        }
    }

    async fn _get_prices(&self, ids: &[&str]) -> Result<Vec<Result<PriceInfo, Error>>, Error> {
        let response = self.send_request(ids).await?;

        let parsed_response = response.json::<CoinMarketResponse>().await?;

        let id_to_response = parsed_response
            .data
            .values()
            .map(|coin_info| (coin_info.slug.clone(), coin_info.clone()))
            .collect::<HashMap<String, CoinInfo>>();

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
                    price: price_info.quote.usd.price,
                    timestamp: datetime.timestamp() as u64,
                })
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    async fn send_request(&self, ids: &[&str]) -> Result<Response, Error> {
        let query: Vec<(&str, String)> = vec![("slug", ids.join(","))];

        let response = self
            .client
            .get(&self.url)
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .header("User-Agent", "BandPriceAdapter")
            .query(&query)
            .send()
            .await?;
        let response_status = response.status();
        if response.status() != StatusCode::OK {
            tracing::trace!(
                "query request get error status {} {}",
                response_status,
                response.text().await?
            );
            return Err(Error::GeneralQueryPriceError());
        }

        Ok(response)
    }
}
