use crate::config::registry::RegistrySource;
use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;
use serde::Deserialize;

/// The configuration for bothan-api's manager.
#[derive(Clone, Debug, Deserialize)]
pub struct CryptoSources {
    pub binance: BinanceWorkerBuilderOpts,
    pub coingecko: CoinGeckoWorkerBuilderOpts,
    // TODO: reimplement others
    // pub bybit: BybitWorkerBuilderOpts,
    // pub coinbase: CoinbaseWorkerBuilderOpts,
    // pub coinmarketcap: CoinMarketCapWorkerBuilderOpts,
    // pub cryptocompare: CryptoCompareWorkerBuilderOpts,
    // pub htx: HtxWorkerBuilderOpts,
    // pub kraken: KrakenWorkerBuilderOpts,
    // pub okx: OkxWorkerBuilderOpts,
}

/// The configuration for bothan-api's crypto asset info manager.
#[derive(Clone, Debug, Deserialize)]
pub struct CryptoInfoManager {
    /// The registry source for the crypto asset info manager.
    pub registry: RegistrySource,
    /// The source configuration for the crypto asset info manager.
    pub source: CryptoSources,
    /// The stale threshold for the crypto asset info.
    pub stale_threshold: i64,
    /// Flag to allow for registry updates through the api
    pub no_update: bool,
}

/// The configuration for all bothan-api's manager.
#[derive(Clone, Debug, Deserialize)]
pub struct ManagerConfig {
    pub crypto: CryptoInfoManager,
}
