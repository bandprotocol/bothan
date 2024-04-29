use config::Config;
use serde::Deserialize;

use bothan_binance::BinanceServiceBuilderOpts;
use bothan_coingecko::CoinGeckoServiceBuilderOpts;
use bothan_coinmarketcap::CoinMarketCapServiceBuilderOpts;
use bothan_cryptocompare::CryptoCompareServiceBuilderOpts;
use bothan_htx::HtxServiceBuilderOpts;
use bothan_kraken::KrakenServiceBuilderOpts;

/// The configuration for the gRPC server.
#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
    pub addr: String,
}

/// The configuration for the data sources.
#[derive(Clone, Debug, Deserialize)]
pub struct SourceConfig {
    pub binance: BinanceServiceBuilderOpts,
    pub coingecko: CoinGeckoServiceBuilderOpts,
    pub coinmarketcap: CoinMarketCapServiceBuilderOpts,
    pub cryptocompare: CryptoCompareServiceBuilderOpts,
    pub htx: HtxServiceBuilderOpts,
    pub kraken: KrakenServiceBuilderOpts,
}

/// The configuration for the registry.
#[derive(Clone, Debug, Deserialize)]
pub struct RegistrySource {
    pub source: String,
    pub version: String,
}

/// The main application configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct RegistryConfig {
    pub crypto_price: RegistrySource,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub source: SourceConfig,
    pub registry: RegistryConfig,
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
