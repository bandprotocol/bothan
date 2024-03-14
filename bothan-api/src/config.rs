use config::Config;
use serde::Deserialize;

use bothan_binance::BinanceServiceBuilderOpts;
use bothan_coingecko::CoinGeckoServiceBuilderOpts;

#[derive(Debug, Deserialize)]
pub struct GrpcConfig {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub binance: BinanceServiceBuilderOpts,
    pub coingecko: CoinGeckoServiceBuilderOpts,
}

#[derive(Debug, Deserialize)]
pub struct RegistryConfig {
    pub source: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub source: SourceConfig,
    pub registry: RegistryConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap();

        // Deserialize the configuration
        config.try_deserialize()
    }
}
