use serde::{Deserialize, Serialize};

use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_bybit::BybitWorkerBuilderOpts;
use bothan_coinbase::CoinbaseWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;
use bothan_coinmarketcap::CoinMarketCapWorkerBuilderOpts;
use bothan_cryptocompare::CryptoCompareWorkerBuilderOpts;
use bothan_htx::HtxWorkerBuilderOpts;
use bothan_kraken::KrakenWorkerBuilderOpts;
use bothan_okx::OkxWorkerBuilderOpts;

/// The configuration for the worker sources.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CryptoSourceConfigs {
    pub binance: Option<BinanceWorkerBuilderOpts>,
    pub bybit: Option<BybitWorkerBuilderOpts>,
    pub coinbase: Option<CoinbaseWorkerBuilderOpts>,
    pub coingecko: Option<CoinGeckoWorkerBuilderOpts>,
    pub coinmarketcap: Option<CoinMarketCapWorkerBuilderOpts>,
    pub cryptocompare: Option<CryptoCompareWorkerBuilderOpts>,
    pub htx: Option<HtxWorkerBuilderOpts>,
    pub kraken: Option<KrakenWorkerBuilderOpts>,
    pub okx: Option<OkxWorkerBuilderOpts>,
}

impl CryptoSourceConfigs {
    pub fn with_default_sources() -> Self {
        CryptoSourceConfigs {
            binance: Some(BinanceWorkerBuilderOpts::default()),
            bybit: Some(BybitWorkerBuilderOpts::default()),
            coinbase: Some(CoinbaseWorkerBuilderOpts::default()),
            coingecko: Some(CoinGeckoWorkerBuilderOpts::default()),
            coinmarketcap: Some(CoinMarketCapWorkerBuilderOpts::default()),
            cryptocompare: Some(CryptoCompareWorkerBuilderOpts::default()),
            htx: Some(HtxWorkerBuilderOpts::default()),
            kraken: Some(KrakenWorkerBuilderOpts::default()),
            okx: Some(OkxWorkerBuilderOpts::default()),
        }
    }
}
