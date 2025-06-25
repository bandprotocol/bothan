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
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
        }
    }
}
