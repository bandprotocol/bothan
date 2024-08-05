use bothan_binance::BinanceWorkerBuilderOpts;
use bothan_coingecko::CoinGeckoWorkerBuilderOpts;
use serde::{Deserialize, Serialize};

/// The configuration for bothan-api's manager.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CryptoSourceConfigs {
    pub binance: BinanceWorkerBuilderOpts,
    pub coingecko: CoinGeckoWorkerBuilderOpts,
}
