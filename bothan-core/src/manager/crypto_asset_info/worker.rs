pub mod opts;

use std::collections::HashMap;

use bothan_lib::registry::{Registry, Valid};
use bothan_lib::metrics::Metrics;
use bothan_lib::store::Store;
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use derive_more::From;
use tracing::{error, info};

use crate::manager::crypto_asset_info::signal_ids::get_source_batched_query_ids;
use crate::manager::crypto_asset_info::worker::opts::CryptoAssetWorkerOpts;

#[derive(From)]
pub enum CryptoAssetWorker {
    Binance(bothan_binance::Worker),
    Bitfinex(bothan_bitfinex::Worker),
    Bybit(bothan_bybit::Worker),
    Coinbase(bothan_coinbase::Worker),
    CoinGecko(bothan_coingecko::Worker),
    CoinMarketCap(bothan_coinmarketcap::Worker),
    Htx(bothan_htx::Worker),
    Kraken(bothan_kraken::Worker),
    Okx(bothan_okx::Worker),
}

#[async_trait::async_trait]
impl AssetWorker for CryptoAssetWorker {
    type Opts = CryptoAssetWorkerOpts;

    fn name(&self) -> &'static str {
        match self {
            CryptoAssetWorker::Binance(w) => w.name(),
            CryptoAssetWorker::Bitfinex(w) => w.name(),
            CryptoAssetWorker::Bybit(w) => w.name(),
            CryptoAssetWorker::Coinbase(w) => w.name(),
            CryptoAssetWorker::CoinGecko(w) => w.name(),
            CryptoAssetWorker::CoinMarketCap(w) => w.name(),
            CryptoAssetWorker::Htx(w) => w.name(),
            CryptoAssetWorker::Kraken(w) => w.name(),
            CryptoAssetWorker::Okx(w) => w.name(),
        }
    }

    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
        metrics: &Metrics,
    ) -> Result<Self, AssetWorkerError> {
        Ok(match opts {
            CryptoAssetWorkerOpts::Binance(opts) => {
                CryptoAssetWorker::from(bothan_binance::Worker::build(opts, store, ids, metrics).await?)
            }
            CryptoAssetWorkerOpts::Bitfinex(opts) => {
                CryptoAssetWorker::from(bothan_bitfinex::Worker::build(opts, store, ids, metrics).await?)
            }
            CryptoAssetWorkerOpts::Bybit(opts) => {
                CryptoAssetWorker::from(bothan_bybit::Worker::build(opts, store, ids, metrics).await?)
            }
            CryptoAssetWorkerOpts::Coinbase(opts) => {
                CryptoAssetWorker::from(bothan_coinbase::Worker::build(opts, store, ids, metrics).await?)
            }
            CryptoAssetWorkerOpts::CoinGecko(opts) => {
                CryptoAssetWorker::from(bothan_coingecko::Worker::build(opts, store, ids, metrics).await?)
            }
            CryptoAssetWorkerOpts::CoinMarketCap(opts) => CryptoAssetWorker::from(
                bothan_coinmarketcap::Worker::build(opts, store, ids, metrics).await?,
            ),
            CryptoAssetWorkerOpts::Htx(opts) => {
                CryptoAssetWorker::from(bothan_htx::Worker::build(opts, store, ids, metrics).await?)
            }
            CryptoAssetWorkerOpts::Kraken(opts) => {
                CryptoAssetWorker::from(bothan_kraken::Worker::build(opts, store, ids, metrics).await?)
            }
            CryptoAssetWorkerOpts::Okx(opts) => {
                CryptoAssetWorker::from(bothan_okx::Worker::build(opts, store, ids, metrics).await?)
            }
        })
    }
}

pub async fn build_workers<S: Store + 'static>(
    registry: &Registry<Valid>,
    opts: &HashMap<String, CryptoAssetWorkerOpts>,
    store: S,
    metrics: &Metrics,
) -> Vec<CryptoAssetWorker> {
    let mut workers = Vec::with_capacity(opts.len());
    for (source_id, query_id) in get_source_batched_query_ids(registry).drain() {
        match opts.get(&source_id) {
            Some(opts) => {
                let ids = query_id.into_iter().collect();
                let builder_callable = CryptoAssetWorker::build(opts.clone(), &store, ids, metrics);
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
