use serde::Deserialize;

use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;

/// The configuration for each data source
#[derive(Clone, Debug, Deserialize)]
pub struct SourceConfig {
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
