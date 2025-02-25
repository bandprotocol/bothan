pub mod opts;

use crate::manager::crypto_asset_info::worker::opts::CryptoAssetWorkerOpts;
use bothan_lib::store::Store;
use bothan_lib::types::AssetState;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::AssetWorker;
use derive_more::From;
use std::collections::HashSet;

#[derive(From)]
pub enum CryptoAssetWorker<S: Store> {
    Binance(bothan_binance::Worker<S>),
    Bitfinex(bothan_bitfinex::Worker<S>),
    Bybit(bothan_bybit::Worker<S>),
    Coinbase(bothan_coinbase::Worker<S>),
    CoinGecko(bothan_coingecko::Worker<S>),
    CoinMarketCap(bothan_coinmarketcap::Worker<S>),
    Htx(bothan_htx::Worker<S>),
    Kraken(bothan_kraken::Worker<S>),
    Okx(bothan_okx::Worker<S>),
}

#[async_trait::async_trait]
impl<S: Store + 'static> AssetWorker<S> for CryptoAssetWorker<S> {
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

    async fn build(opts: Self::Opts, store: &S) -> Result<Self, AssetWorkerError> {
        Ok(match opts {
            CryptoAssetWorkerOpts::Binance(opts) => {
                CryptoAssetWorker::from(bothan_binance::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::Bitfinex(opts) => {
                CryptoAssetWorker::from(bothan_bitfinex::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::Bybit(opts) => {
                CryptoAssetWorker::from(bothan_bybit::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::Coinbase(opts) => {
                CryptoAssetWorker::from(bothan_coinbase::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::CoinGecko(opts) => {
                CryptoAssetWorker::from(bothan_coingecko::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::CoinMarketCap(opts) => {
                CryptoAssetWorker::from(bothan_coinmarketcap::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::Htx(opts) => {
                CryptoAssetWorker::from(bothan_htx::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::Kraken(opts) => {
                CryptoAssetWorker::from(bothan_kraken::Worker::build(opts, store).await?)
            }
            CryptoAssetWorkerOpts::Okx(opts) => {
                CryptoAssetWorker::from(bothan_okx::Worker::build(opts, store).await?)
            }
        })
    }

    async fn get_asset(&self, id: &str) -> Result<AssetState, AssetWorkerError> {
        match self {
            CryptoAssetWorker::Binance(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::Bitfinex(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::Bybit(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::Coinbase(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::CoinGecko(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::CoinMarketCap(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::Htx(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::Kraken(worker) => worker.get_asset(id).await,
            CryptoAssetWorker::Okx(worker) => worker.get_asset(id).await,
        }
    }

    async fn set_query_ids(&self, ids: HashSet<String>) -> Result<(), AssetWorkerError> {
        match self {
            CryptoAssetWorker::Binance(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::Bitfinex(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::Bybit(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::Coinbase(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::CoinGecko(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::CoinMarketCap(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::Htx(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::Kraken(worker) => worker.set_query_ids(ids).await,
            CryptoAssetWorker::Okx(worker) => worker.set_query_ids(ids).await,
        }
    }
}
