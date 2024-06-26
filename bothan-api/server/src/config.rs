use config::Config;
use serde::Deserialize;

use bothan_binance::BinanceServiceBuilderOpts;
use bothan_bybit::BybitServiceBuilderOpts;
use bothan_coinbase::CoinbaseServiceBuilderOpts;
use bothan_coingecko::CoinGeckoServiceBuilderOpts;
use bothan_coinmarketcap::CoinMarketCapServiceBuilderOpts;
use bothan_cryptocompare::CryptoCompareServiceBuilderOpts;
use bothan_htx::HtxServiceBuilderOpts;
use bothan_kraken::KrakenServiceBuilderOpts;
use bothan_okx::OkxServiceBuilderOpts;

/// The configuration for the gRPC server.
#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
    pub addr: String,
}

/// The configuration for the manager.
#[derive(Clone, Debug, Deserialize)]
pub struct ManagerConfig {
    pub stale_threshold: u64,
}

/// The configuration for each data source
#[derive(Clone, Debug, Deserialize)]
pub struct SourceConfig {
    pub binance: BinanceServiceBuilderOpts,
    pub bybit: BybitServiceBuilderOpts,
    pub coinbase: CoinbaseServiceBuilderOpts,
    pub coingecko: CoinGeckoServiceBuilderOpts,
    pub coinmarketcap: CoinMarketCapServiceBuilderOpts,
    pub cryptocompare: CryptoCompareServiceBuilderOpts,
    pub htx: HtxServiceBuilderOpts,
    pub kraken: KrakenServiceBuilderOpts,
    pub okx: OkxServiceBuilderOpts,
}

/// The configuration for the registry.
#[derive(Clone, Debug, Deserialize)]
pub struct RegistrySourceConfig {
    pub source: String,
    pub version: String,
}

/// The registry source configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct RegistryConfig {
    pub crypto_price: RegistrySourceConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

/// The main application configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub manager: ManagerConfig,
    pub source: SourceConfig,
    pub registry: RegistryConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    /// Creates a new `AppConfig` using the configuration file.
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap();

        // Deserialize the configuration
        config.try_deserialize()
    }
}
