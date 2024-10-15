use std::sync::Weak;
use std::time::Duration;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::time::{interval, timeout};
use tracing::{debug, error, trace, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::rest::CryptoCompareRestAPI;
use crate::worker::error::ParseError;
use crate::worker::CryptoCompareWorker;

pub(crate) fn start_asset_worker(
    weak_worker: Weak<CryptoCompareWorker>,
    update_interval: Duration,
) {
    let mut interval = interval(update_interval);
    tokio::spawn(async move {
        while let Some(worker) = weak_worker.upgrade() {
            interval.tick().await;

            let ids = match worker.store.get_query_ids().await {
                Ok(ids) => ids.into_iter().collect::<Vec<String>>(),
                Err(e) => {
                    error!("failed to get query ids with error: {}", e);
                    Vec::new()
                }
            };

            if ids.is_empty() {
                debug!("no ids to update, skipping update");
                continue;
            }

            let result = timeout(
                interval.period(),
                update_asset_info(&worker.store, &worker.api, &ids),
            )
            .await;

            if result.is_err() {
                warn!("updating interval exceeded timeout")
            }
        }

        debug!("asset worker has been dropped, stopping asset worker");
    });
}

async fn update_asset_info<T: AsRef<str>>(
    store: &WorkerStore,
    api: &CryptoCompareRestAPI,
    ids: &[T],
) {
    let now = chrono::Utc::now().timestamp();
    // Convert ids to a slice of &str
    let ids_str: Vec<&str> = ids.iter().map(|id| id.as_ref()).collect();

    match api.get_multi_symbol_price(&ids_str).await {
        Ok(prices) => {
            let mut to_set = Vec::new();

            for (id, price) in ids_str.iter().zip(prices.iter()) {
                let price = match price {
                    Some(price) => price,
                    None => {
                        warn!("missing price data for id: {}", id);
                        continue;
                    }
                };

                match parse_symbol_price(id, price, &now) {
                    Ok(asset_info) => to_set.push((id.to_string(), asset_info)),
                    Err(_) => warn!("failed to parse price data for id: {}", id),
                }
            }

            // Set multiple assets at once
            if let Err(e) = store.set_assets(to_set.clone()).await {
                warn!("failed to set multiple assets with error: {}", e);
            } else {
                trace!(
                    "stored data for ids: {:?}",
                    to_set.iter().map(|(id, _)| id).collect::<Vec<&String>>(),
                );
            }
        }
        Err(e) => warn!("failed to get price data with error: {}", e),
    }
}

fn parse_symbol_price(
    id: &str,
    symbol_price: &f64,
    timestamp: &i64,
) -> Result<AssetInfo, ParseError> {
    let price_value = Decimal::from_f64(symbol_price.to_owned())
        .ok_or(ParseError::InvalidPrice(symbol_price.to_owned()))?;
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

    #[test]
    fn test_parse_symbol_price() {
        let id = "BTC";
        let price = 8426.69;
        let timestamp = 1609459200;

        // Call parse_symbol_price with the updated parameters
        let result = parse_symbol_price(id, &price, &timestamp);
        let expected = AssetInfo::new(
            id.to_string(),
            Decimal::from_str("8426.69").unwrap(),
            timestamp,
        );
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_symbol_price_with_failure() {
        let id = "BTC";
        let price = f64::INFINITY; // Invalid price value
        let timestamp = 0;

        // Test failure case where the price is invalid
        assert!(parse_symbol_price(id, &price, &timestamp).is_err());
    }
}
