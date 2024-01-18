use std::collections::HashMap;

use chrono::Utc;
use reqwest::{Client, Response, StatusCode};
use serde::Deserialize;

use crate::{error::Error, types::PriceInfo};

const ENDPOINT: &str = "https://api.huobi.pro/";

#[derive(Deserialize, Debug, Clone)]
pub struct CoinMarketResponse {
    pub id: String,
    pub close: f64,
}

/// Base object to query HuobiPro api.
#[derive(Default)]
pub struct HuobiPro {
    url: String,
    client: Client,
}

impl HuobiPro {
    // initiate new api object.
    pub fn new() -> Self {
        Self {
            url: ENDPOINT.into(),
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
        let response = self.send_request().await?;

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

                Ok(PriceInfo {
                    id: id.to_string(),
                    price: price_info.close,
                    timestamp: Utc::now().timestamp() as u64,
                })
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    async fn send_request(&self) -> Result<Response, Error> {
        let url = format!("{}market/tickers", self.url);

        let response = self
            .client
            .get(url)
            .header("User-Agent", "BandPriceAdapter")
            .send()
            .await?;
        let response_status = response.status();
        if response_status != StatusCode::OK {
            tracing::trace!(
                "query request get error status {} {}",
                response_status,
                response.text().await?
            );
            return Err(Error::ResponseStatusNotOk(response_status));
        }

        Ok(response)
    }
}
