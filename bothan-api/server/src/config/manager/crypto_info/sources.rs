use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;
use serde::Deserialize;

/// The configuration for bothan-api's manager.
#[derive(Clone, Debug, Deserialize)]
pub struct CryptoSourceConfigs {
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
