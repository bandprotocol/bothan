use std::sync::Weak;
use std::time::Duration;

use chrono::NaiveDateTime;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::time::{interval, timeout, Interval};
use tracing::{info, warn};

use bothan_core::store::Store;
use bothan_core::types::AssetInfo;

use crate::api::rest::CoinGeckoRestAPI;
use crate::api::types::{Market, Order};
use crate::worker::error::ParseError;
use crate::worker::CoinGeckoWorker;

pub(crate) fn start_asset_worker(
    weak_worker: Weak<CoinGeckoWorker>,
    mut update_interval: Interval,
    page_size: usize,
    page_query_delay: Duration,
) {
    tokio::spawn(async move {
        while let Some(worker) = weak_worker.upgrade() {
            info!("updating asset info");

            let ids = worker.store.get_query_ids().await;

            let result = timeout(
                page_query_delay,
                update_all_asset_info(&worker.store, &worker.api, ids, page_size, page_query_delay),
            )
            .await;

            if result.is_err() {
                warn!("updating interval exceeded timeout")
            }

            update_interval.tick().await;
        }

        info!("worker has been dropped, stopping asset worker");
    });
}

async fn update_all_asset_info(
    store: &Store,
    api: &CoinGeckoRestAPI,
    ids: Vec<String>,
    page_size: usize,
    page_delay: Duration,
) {
    let mut interval = interval(page_delay);
    let queue = Vec::from_iter(ids.chunks(page_size));
    for ids in queue {
        update_asset_info(store, api, ids, page_size).await;
        interval.tick().await;
    }
}

async fn update_asset_info(
    store: &Store,
    api: &CoinGeckoRestAPI,
    ids: &[String],
    page_size: usize,
) {
    match api
        .get_coins_market(ids, Some(Order::IdDesc), Some(page_size), Some(1))
        .await
    {
        Ok(markets) => {
            let to_set = markets
                .into_iter()
                .filter_map(|market| {
                    let id = market.id.clone();
                    match parse_market(market) {
                        Ok(asset_info) => Some((id, asset_info)),
                        Err(e) => {
                            warn!("failed to parse market data for {} with error {}", id, e);
                            None
                        }
                    }
                })
                .collect::<Vec<(String, AssetInfo)>>();
            store.set_assets(to_set).await;
        }
        Err(e) => {
            warn!(
                "failed to get market data for ids '{:?}' with error: {}",
                ids, e
            );
        }
    }
}

fn parse_market(market: Market) -> Result<AssetInfo, ParseError> {
    let last_updated = market.last_updated.as_str();
    let ts = NaiveDateTime::parse_from_str(last_updated, "%Y-%m-%dT%H:%M:%S.%fZ")?
        .and_utc()
        .timestamp() as u64;

    let price = Decimal::from_f64(market.current_price)
        .ok_or(ParseError::InvalidPrice(market.current_price))?;

    Ok(AssetInfo::new(market.id, price, ts))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_market() {
        let market = Market {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 8426.69,
            last_updated: "2021-01-01T00:00:00.000Z".to_string(),
        };
        let result = parse_market(market);
        let expected = AssetInfo::new(
            "bitcoin".to_string(),
            Decimal::from_str_exact("8426.69").unwrap(),
            1609459200,
        );
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_market_with_failure() {
        let market = Market {
            id: "bitcoin".to_string(),
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            current_price: 8426.69,
            last_updated: "johnny appleseed".to_string(),
        };
        assert!(parse_market(market).is_err());
    }
}
