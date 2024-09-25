use std::sync::Weak;
use std::time::Duration;

use chrono::NaiveDateTime;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::time::{interval, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::rest::CoinMarketCapRestAPI;
use crate::api::types::Quote;
use crate::worker::error::ParseError;
use crate::worker::CoinMarketCapWorker;

pub(crate) fn start_asset_worker(
    weak_worker: Weak<CoinMarketCapWorker>,
    update_interval: Duration,
) {
    let mut interval = interval(update_interval);
    tokio::spawn(async move {
        while let Some(worker) = weak_worker.upgrade() {
            info!("updating asset info");

            let ids = match worker.store.get_query_ids().await {
                Ok(ids) => ids.into_iter().collect::<Vec<String>>(),
                Err(e) => {
                    error!("failed to get query ids with error: {}", e);
                    Vec::new()
                }
            };

            let result = timeout(
                interval.period(),
                update_asset_info(&worker.store, &worker.api, &ids),
            )
            .await;

            if result.is_err() {
                warn!("updating interval exceeded timeout")
            }

            interval.tick().await;
        }

        debug!("asset worker has been dropped, stopping asset worker");
    });
}

async fn update_asset_info<T: AsRef<str>>(
    store: &WorkerStore,
    api: &CoinMarketCapRestAPI,
    ids: &[T],
) {
    // Convert ids to a slice of &str
    let ids_str: Vec<&str> = ids.iter().map(|id| id.as_ref()).collect();

    match api.get_latest_quotes(&ids_str).await {
        Ok(quotes) => {
            let mut to_set = Vec::new();

            for (id, quote) in ids_str.iter().zip(quotes.iter()) {
                let quote = match quote {
                    Some(price) => price,
                    None => {
                        warn!("missing price data for id: {}", id);
                        continue;
                    }
                };

                match parse_quote(id, quote) {
                    Ok(asset_info) => to_set.push((id.to_string(), asset_info)),
                    Err(_) => warn!("failed to parse price data for id: {}", id),
                }
            }

            // Set multiple assets at once
            if let Err(e) = store.set_assets(to_set).await {
                warn!("failed to set multiple assets with error: {}", e);
            }
        }
        Err(e) => warn!("failed to get price data with error: {}", e),
    }
}

pub(crate) fn parse_quote(id: &str, quote: &Quote) -> Result<AssetInfo, ParseError> {
    let price = quote
        .price_quotes
        .usd
        .price
        .ok_or(ParseError::MissingPriceData)?;
    let price_value =
        Decimal::from_f64(price.to_owned()).ok_or(ParseError::InvalidPrice(price.to_owned()))?;
    let last_updated = quote.price_quotes.usd.last_updated.as_str();
    let naive_date_time = NaiveDateTime::parse_from_str(last_updated, "%Y-%m-%dT%H:%M:%S.%fZ")?;
    let timestamp = naive_date_time.and_utc().timestamp();

    Ok(AssetInfo::new(
        id.to_owned(),
        price_value,
        timestamp.to_owned(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    use crate::api::types::{PriceQuote, PriceQuotes};

    #[test]
    fn test_parse_quote() {
        let id = "1";
        let price = 8426.69;
        let timestamp = "2021-01-01T00:00:00.000Z";

        let quote = Quote {
            id: 1 as usize,
            symbol: "BTC".to_string(),
            slug: "bitcoin".to_string(),
            name: "Bitcoin".to_string(),
            price_quotes: PriceQuotes {
                usd: PriceQuote {
                    price: Some(price),
                    volume_24h: 123.0,
                    volume_change_24h: 456.0,
                    market_cap: 789.0,
                    market_cap_dominance: 0.0,
                    fully_diluted_market_cap: 0.0,
                    percent_change_1h: 0.0,
                    percent_change_24h: 0.0,
                    percent_change_7d: 0.0,
                    percent_change_30d: 0.0,
                    last_updated: timestamp.to_string(),
                },
            },
        };

        let asset_info = parse_quote(id, &quote).unwrap();
        assert_eq!(asset_info.id, id);
        assert_eq!(asset_info.price, Decimal::from_str("8426.69").unwrap());
        assert_eq!(asset_info.timestamp, 1609459200);
    }

    #[test]
    fn test_parse_quote_with_failure() {
        let id = "1";
        let price = f64::INFINITY;
        let timestamp = "2021-01-01T00:00:00.000Z";

        let quote = Quote {
            id: 1 as usize,
            symbol: "BTC".to_string(),
            slug: "bitcoin".to_string(),
            name: "Bitcoin".to_string(),
            price_quotes: PriceQuotes {
                usd: PriceQuote {
                    price: Some(price),
                    volume_24h: 123.0,
                    volume_change_24h: 456.0,
                    market_cap: 789.0,
                    market_cap_dominance: 0.0,
                    fully_diluted_market_cap: 0.0,
                    percent_change_1h: 0.0,
                    percent_change_24h: 0.0,
                    percent_change_7d: 0.0,
                    percent_change_30d: 0.0,
                    last_updated: timestamp.to_string(),
                },
            },
        };

        assert!(parse_quote(id, &quote).is_err());
    }
}
