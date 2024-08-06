use serde::{Deserialize, Serialize};

use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;

/// The configuration for the worker sources.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CryptoSourceConfigs {
    pub binance: BinanceWorkerBuilderOpts,
    pub coingecko: CoinGeckoWorkerBuilderOpts,
}
