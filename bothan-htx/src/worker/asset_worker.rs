use std::sync::Weak;
use std::time::Duration;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::time::{interval, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::rest::HtxRestAPI;
use crate::api::types::Ticker;
use crate::worker::error::ParseError;
use crate::worker::HtxWorker;

pub(crate) fn start_asset_worker(weak_worker: Weak<HtxWorker>, update_interval: Duration) {
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

use std::collections::HashSet;

async fn update_asset_info<T: AsRef<str>>(store: &WorkerStore, api: &HtxRestAPI, ids: &[T]) {
    // Convert the slice of ids into a HashSet for faster lookups
    let id_set: HashSet<&str> = ids.iter().map(|id| id.as_ref()).collect();

    match api.get_latest_tickers().await {
        Ok(quote) => {
            let timestamp = quote.timestamp;
            let to_set = quote
                .data
                .into_iter()
                .filter_map(|ticker| {
                    // Check if the ticker symbol is in the HashSet
                    if id_set.contains(ticker.symbol.as_str()) {
                        match parse_ticker(&ticker, timestamp) {
                            Ok(asset_info) => Some((ticker.symbol, asset_info)),
                            Err(e) => {
                                warn!(
                                    "failed to parse market data for {} with error {}",
                                    ticker.symbol, e
                                );
                                None
                            }
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<(String, AssetInfo)>>();

            if let Err(e) = store.set_assets(to_set).await {
                error!("failed to set asset info with error: {}", e);
            }
        }
        Err(e) => {
            warn!("failed to get market data with error: {}", e);
        }
    }
}

pub fn parse_ticker(ticker: &Ticker, timestamp: usize) -> Result<AssetInfo, ParseError> {
    let price_value =
        Decimal::from_f64(ticker.close).ok_or(ParseError::InvalidPrice(ticker.close))?;
    Ok(AssetInfo::new(
        ticker.symbol.to_string(),
        price_value,
        timestamp as i64,
    ))
}
