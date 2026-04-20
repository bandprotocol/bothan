//! Worker trait and implementations for asset sources.

pub mod opts;

use std::collections::HashMap;

use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use derive_more::From;
use tracing::{error, info};

use crate::manager::asset_info::signal_ids::get_source_batched_query_ids;
use crate::manager::asset_info::worker::opts::AssetWorkerOpts;

#[derive(From)]
pub enum AnyAssetWorker {
    Binance(bothan_binance::Worker),
    Bitfinex(bothan_bitfinex::Worker),
    Bybit(bothan_bybit::Worker),
    Coinbase(bothan_coinbase::Worker),
    CoinGecko(bothan_coingecko::Worker),
    CoinMarketCap(bothan_coinmarketcap::Worker),
    Htx(bothan_htx::Worker),
    Kraken(bothan_kraken::Worker),
    Okx(bothan_okx::Worker),
    Band(bothan_band::Worker),
}

#[async_trait::async_trait]
impl AssetWorker for AnyAssetWorker {
    type Opts = AssetWorkerOpts;

    fn name(&self) -> &'static str {
        match self {
            AnyAssetWorker::Binance(w) => w.name(),
            AnyAssetWorker::Bitfinex(w) => w.name(),
            AnyAssetWorker::Bybit(w) => w.name(),
            AnyAssetWorker::Coinbase(w) => w.name(),
            AnyAssetWorker::CoinGecko(w) => w.name(),
            AnyAssetWorker::CoinMarketCap(w) => w.name(),
            AnyAssetWorker::Htx(w) => w.name(),
            AnyAssetWorker::Kraken(w) => w.name(),
            AnyAssetWorker::Okx(w) => w.name(),
            AnyAssetWorker::Band(w) => w.name(),
        }
    }

    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError> {
        Ok(match opts {
            AssetWorkerOpts::Binance(opts) => {
                AnyAssetWorker::from(bothan_binance::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::Bitfinex(opts) => {
                AnyAssetWorker::from(bothan_bitfinex::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::Bybit(opts) => {
                AnyAssetWorker::from(bothan_bybit::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::Coinbase(opts) => {
                AnyAssetWorker::from(bothan_coinbase::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::CoinGecko(opts) => {
                AnyAssetWorker::from(bothan_coingecko::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::CoinMarketCap(opts) => {
                AnyAssetWorker::from(bothan_coinmarketcap::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::Htx(opts) => {
                AnyAssetWorker::from(bothan_htx::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::Kraken(opts) => {
                AnyAssetWorker::from(bothan_kraken::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::Okx(opts) => {
                AnyAssetWorker::from(bothan_okx::Worker::build(opts, store, ids).await?)
            }
            AssetWorkerOpts::Band(opts) => {
                AnyAssetWorker::from(bothan_band::Worker::build(opts, store, ids).await?)
            }
        })
    }
}

pub async fn build_workers<S: Store + 'static>(
    registry: &Registry<Valid>,
    opts: &HashMap<String, AssetWorkerOpts>,
    store: S,
) -> Vec<AnyAssetWorker> {
    let mut workers = Vec::with_capacity(opts.len());
    for (source_id, query_id) in get_source_batched_query_ids(registry).drain() {
        match opts.get(&source_id) {
            Some(opts) => {
                let ids = query_id.into_iter().collect();
                let builder_callable = AnyAssetWorker::build(opts.clone(), &store, ids);
                let worker = match builder_callable.await {
                    Ok(worker) => worker,
                    Err(e) => {
                        error!("failed to build worker {}: {}", source_id, e);
                        continue;
                    }
                };
                workers.push(worker);
            }
            None => info!("worker {} not activated", source_id),
        }
    }

    workers
}
