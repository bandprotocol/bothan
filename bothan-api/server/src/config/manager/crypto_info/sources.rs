use serde::{Deserialize, Serialize};

use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;

/// The configuration for the worker sources.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CryptoSourceConfigs {
    pub binance: SourceConfig<BinanceWorkerBuilderOpts>,
    pub coingecko: SourceConfig<CoinGeckoWorkerBuilderOpts>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourceConfig<T> {
    #[serde(flatten)]
    pub builder_opts: T,
    pub skip: bool,
}
