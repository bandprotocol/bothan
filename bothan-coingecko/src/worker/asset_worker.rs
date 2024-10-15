use std::sync::Weak;
use std::time::Duration;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::time::{interval, timeout};
use tracing::{debug, error, trace, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::rest::CoinGeckoRestAPI;
use crate::api::types::Price;
use crate::worker::error::ParseError;
use crate::worker::CoinGeckoWorker;

pub(crate) fn start_asset_worker(weak_worker: Weak<CoinGeckoWorker>, update_interval: Duration) {
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

async fn update_asset_info<T: AsRef<str>>(store: &WorkerStore, api: &CoinGeckoRestAPI, ids: &[T]) {
    match api.get_simple_price_usd(ids).await {
        Ok(markets) => {
            // Sanity check to assure that the number of markets returned is less than the number of ids
            if markets.len() <= ids.len() {
                let to_set = markets
                    .into_iter()
                    .filter_map(|(id, price)| match parse_price(&id, price) {
                        Ok(asset_info) => Some((id, asset_info)),
                        Err(e) => {
                            warn!("failed to parse market data for {} with error {}", id, e);
                            None
                        }
                    })
                    .collect::<Vec<(String, AssetInfo)>>();
                if let Err(e) = store.set_assets(to_set.clone()).await {
                    error!("failed to set asset info with error: {}", e);
                } else {
                    trace!(
                        "stored data for ids: {:?}",
                        to_set.iter().map(|(id, _)| id).collect::<Vec<&String>>(),
                    );
                }
            } else {
                warn!(
                    "received more markets than ids, ids: {}, markets: {}",
                    ids.len(),
                    markets.len()
                );
            }
        }
        Err(e) => {
            warn!("failed to get market data with error: {}", e);
        }
    }
}

fn parse_price<T: Into<String>>(id: T, price: Price) -> Result<AssetInfo, ParseError> {
    let price_value = Decimal::from_f64(price.usd).ok_or(ParseError::InvalidPrice(price.usd))?;
    Ok(AssetInfo::new(
        id.into(),
        price_value,
        price.last_updated_at,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_market() {
        let price = Price {
            usd: 8426.69,
            last_updated_at: 1609459200,
        };
        let result = parse_price("bitcoin", price);
        let expected = AssetInfo::new(
            "bitcoin".to_string(),
            Decimal::from_str_exact("8426.69").unwrap(),
            1609459200,
        );
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_market_with_failure() {
        let price = Price {
            usd: f64::INFINITY,
            last_updated_at: 0,
        };
        assert!(parse_price("bitcoin", price).is_err());
    }
}
