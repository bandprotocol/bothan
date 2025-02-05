use serde::{Deserialize, Serialize};

/// The configuration for the worker sources.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CryptoSourceConfigs {
    pub binance: Option<bothan_binance::WorkerOpts>,
    pub bitfinex: Option<bothan_bitfinex::WorkerOpts>,
    pub bybit: Option<bothan_bybit::WorkerOpts>,
    pub coinbase: Option<bothan_coinbase::WorkerOpts>,
    pub coingecko: Option<bothan_coingecko::WorkerOpts>,
    pub coinmarketcap: Option<bothan_coinmarketcap::WorkerOpts>,
    pub htx: Option<bothan_htx::WorkerOpts>,
    pub kraken: Option<bothan_kraken::WorkerOpts>,
    pub okx: Option<bothan_okx::WorkerOpts>,
}

impl CryptoSourceConfigs {
    pub fn with_default_sources() -> Self {
        CryptoSourceConfigs {
            binance: Some(bothan_binance::WorkerOpts::default()),
            bitfinex: Some(bothan_bitfinex::WorkerOpts::default()),
            bybit: Some(bothan_bybit::WorkerOpts::default()),
            coinbase: Some(bothan_coinbase::WorkerOpts::default()),
            coingecko: Some(bothan_coingecko::WorkerOpts::default()),
            coinmarketcap: Some(bothan_coinmarketcap::WorkerOpts::default()),
            htx: Some(bothan_htx::WorkerOpts::default()),
            kraken: Some(bothan_kraken::WorkerOpts::default()),
            okx: Some(bothan_okx::WorkerOpts::default()),
        }
    }
}
