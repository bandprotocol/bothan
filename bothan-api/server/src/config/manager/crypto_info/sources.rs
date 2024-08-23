use serde::{Deserialize, Serialize};

use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;
use bothan_kraken::KrakenWorkerBuilderOpts;

/// The configuration for the worker sources.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CryptoSourceConfigs {
    pub binance: Option<BinanceWorkerBuilderOpts>,
    pub coingecko: Option<CoinGeckoWorkerBuilderOpts>,
    pub kraken: Option<KrakenWorkerBuilderOpts>,
}

impl CryptoSourceConfigs {
    pub fn with_default_sources() -> Self {
        CryptoSourceConfigs {
            binance: Some(BinanceWorkerBuilderOpts::default()),
            coingecko: Some(CoinGeckoWorkerBuilderOpts::default()),
            kraken: Some(KrakenWorkerBuilderOpts::default()),
        }
    }
}
