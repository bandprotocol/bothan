//! Bothan API server crypto source configuration.
//!
//! Worker options for supported crypto data sources.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
//! let sources = CryptoSourceConfigs::with_default_sources();
//! ```

use serde::{Deserialize, Serialize};

/// Configuration for the worker sources for crypto asset info.
#[derive(Clone, Debug, Default, Serialize)]
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
    pub band1: Option<bothan_band::WorkerOpts>,
    /// Band2 worker options.
    pub band2: Option<bothan_band::WorkerOpts>,
}

impl<'de> Deserialize<'de> for CryptoSourceConfigs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            binance: Option<bothan_binance::WorkerOpts>,
            bitfinex: Option<bothan_bitfinex::WorkerOpts>,
            bybit: Option<bothan_bybit::WorkerOpts>,
            coinbase: Option<bothan_coinbase::WorkerOpts>,
            coingecko: Option<bothan_coingecko::WorkerOpts>,
            coinmarketcap: Option<bothan_coinmarketcap::WorkerOpts>,
            htx: Option<bothan_htx::WorkerOpts>,
            kraken: Option<bothan_kraken::WorkerOpts>,
            okx: Option<bothan_okx::WorkerOpts>,
            band1: Option<bothan_band::WorkerOpts>,
            band2: Option<bothan_band::WorkerOpts>,
        }

        let mut helper = Helper::deserialize(deserializer)?;

        // Custom logic to modify `band1` during deserialization
        if let Some(ref mut band1) = helper.band1 {
            band1.name = Some("band1".to_string());
        }
        // Custom logic to modify `band2` during deserialization
        if let Some(ref mut band2) = helper.band2 {
            band2.name = Some("band2".to_string());
        }

        Ok(CryptoSourceConfigs {
            binance: helper.binance,
            bitfinex: helper.bitfinex,
            bybit: helper.bybit,
            coinbase: helper.coinbase,
            coingecko: helper.coingecko,
            coinmarketcap: helper.coinmarketcap,
            htx: helper.htx,
            kraken: helper.kraken,
            okx: helper.okx,
            band1: helper.band1,
            band2: helper.band2,
        })
    }
}

impl CryptoSourceConfigs {
    /// Creates a new `CryptoSourceConfigs` with all sources set to their default options.
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
            band1: Some(bothan_band::WorkerOpts::new("band1", "https://bandsource1.bandchain.org")),
            band2: Some(bothan_band::WorkerOpts::new("band2", "https://bandsource1.bandchain.org")),
        }
    }
}
