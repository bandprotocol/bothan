//! Bothan API server crypto source configuration.
//!
//! Worker options for supported crypto data sources.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
//! let sources = CryptoSourceConfigs::default();
//! ```

use serde::{Deserialize, Serialize};

/// Configuration for the worker sources for crypto asset info.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoSourceConfigs {
    /// Binance worker options.
    pub binance: Option<bothan_binance::WorkerOpts>,
    /// Bitfinex worker options.
    pub bitfinex: Option<bothan_bitfinex::WorkerOpts>,
    /// Bybit worker options.
    pub bybit: Option<bothan_bybit::WorkerOpts>,
    /// Coinbase worker options.
    pub coinbase: Option<bothan_coinbase::WorkerOpts>,
    /// Coingecko worker options.
    pub coingecko: Option<bothan_coingecko::WorkerOpts>,
    /// Coinmarketcap worker options.
    pub coinmarketcap: Option<bothan_coinmarketcap::WorkerOpts>,
    /// HTX worker options.
    pub htx: Option<bothan_htx::WorkerOpts>,
    /// Kraken worker options.
    pub kraken: Option<bothan_kraken::WorkerOpts>,
    /// OKX worker options.
    pub okx: Option<bothan_okx::WorkerOpts>,
    /// Band1 worker options.
    #[serde(deserialize_with = "de_band1")]
    pub band1: Option<bothan_band::WorkerOpts>,
    /// Band2 worker options.
    #[serde(deserialize_with = "de_band2")]
    pub band2: Option<bothan_band::WorkerOpts>,
}

macro_rules! de_band_named {
    ($fn_name:ident, $name:expr) => {
        fn $fn_name<'de, D>(d: D) -> Result<Option<bothan_band::WorkerOpts>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let mut v = Option::<bothan_band::WorkerOpts>::deserialize(d)?;
            if let Some(ref mut w) = v {
                w.name = $name;
            }
            Ok(v)
        }
    };
}

const BAND1_WORKER_NAME: &str = "band1";
de_band_named!(de_band1, BAND1_WORKER_NAME);
const BAND2_WORKER_NAME: &str = "band2";
de_band_named!(de_band2, BAND2_WORKER_NAME);

impl Default for CryptoSourceConfigs {
    fn default() -> Self {
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
            band1: Some(bothan_band::WorkerOpts::new(
                "band1",
                "https://bandsource1.bandchain.org",
            )),
            band2: Some(bothan_band::WorkerOpts::new(
                "band2",
                "https://bandsource2.bandchain.org",
            )),
        }
    }
}
