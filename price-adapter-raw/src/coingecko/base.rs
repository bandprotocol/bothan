use std::collections::HashMap;

use itertools::Itertools;
use reqwest::{Client, Response, StatusCode};
use serde::Deserialize;

use crate::{error::Error, types::PriceInfo};

// id can get from https://www.coingecko.com/api/documentation.

const USD: &str = "USD";

#[derive(Deserialize, Debug, Clone)]
pub struct CoinMarketResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
}

/// Base object to query Coingecko api.
#[derive(Default)]
pub struct CoingeckoBase {
    api_key: String,
    url: String,
    client: Client,
}

impl CoingeckoBase {
    /// initiate new api object.
    pub fn new(url: String) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }

    /// set up an api key.
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }

    async fn _get_prices(
        &self,
        symbol_ids: &[(&str, &str)],
    ) -> Result<Vec<Result<PriceInfo, Error>>, Error> {
        let unique_ids = self.get_unique_ids(symbol_ids);

        let timestamp = chrono::Utc::now().timestamp() as u64;
        let response = self.send_request(&unique_ids).await?;

        let id_to_prices = response
            .json::<Vec<CoinMarketResponse>>()
            .await?
            .into_iter()
            .map(|item| (item.id, item.current_price))
            .collect::<HashMap<String, f64>>();

        let prices = self.calculate_pair_prices(symbol_ids, id_to_prices);

        let results = symbol_ids
            .iter()
            .zip(prices.into_iter())
            .map(|((base, quote), price)| {
                price.map(|price| PriceInfo {
                    base: base.to_string(),
                    quote: quote.to_string(),
                    price,
                    timestamp,
                })
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    async fn send_request(&self, unique_ids: &[&str]) -> Result<Response, Error> {
        let mut query = vec![("ids", unique_ids.join(",")), ("vs_currency", "usd".into())];
        if !self.api_key.is_empty() {
            query.push(("x_cg_pro_api_key", self.api_key.clone()))
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

    fn get_unique_ids<'a>(&self, symbol_ids: &[(&'a str, &'a str)]) -> Vec<&'a str> {
        let (base_ids, quote_ids): (Vec<&str>, Vec<&str>) =
            symbol_ids.iter().map(|(base, quote)| (base, quote)).unzip();

        base_ids
            .iter()
            .chain(quote_ids.iter())
            .unique()
            .filter(|&&id| id != USD)
            .copied()
            .collect::<Vec<_>>()
    }

    fn calculate_pair_prices(
        &self,
        symbol_ids: &[(&str, &str)],
        id_to_prices: HashMap<String, f64>,
    ) -> Vec<Result<f64, Error>> {
        symbol_ids
            .iter()
            .map(|(base, quote)| {
                let base_price = id_to_prices.get(*base).copied();
                let quote_price = id_to_prices.get(*quote).copied();

                match (*quote, base_price, quote_price) {
                    (USD, Some(price), _) => Ok(price),
                    (_, None, _) | (_, _, None) => Err(Error::NotFound((*base).into())),
                    (_, Some(base_price), Some(quote_price)) => {
                        if quote_price != 0.0 {
                            Ok(base_price / quote_price)
                        } else {
                            Err(Error::ZeroPrice((*quote).into()))
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
    }

    /// get pair prices from the given queries (list of a tuple of (base, quote)).
    pub async fn get_prices(&self, symbol_ids: &[(&str, &str)]) -> Vec<Result<PriceInfo, Error>> {
        match self._get_prices(symbol_ids).await {
            Ok(results) => results,
            Err(err) => {
                tracing::error!("get prices error: {}", err);
                symbol_ids
                    .iter()
                    .map(|_| Err(Error::GeneralQueryPriceError()))
                    .collect()
            }
        }
    }

    /// get a pair price from the given query.
    pub async fn get_price(&self, base: &str, quote: &str) -> Result<PriceInfo, Error> {
        let symbol_ids = vec![(base, quote)];
        let mut prices = self.get_prices(&symbol_ids).await;

        prices
            .pop()
            .ok_or(Error::NotFound(format!("{}/{}", base, quote)))?
    }
}
