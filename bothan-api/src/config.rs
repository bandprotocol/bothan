use config::Config;
use serde::Deserialize;

use bothan_binance::BinanceServiceBuilderOpts;
use bothan_coingecko::CoinGeckoServiceBuilderOpts;
use bothan_coinmarketcap::CoinMarketCapServiceBuilderOpts;
use bothan_cryptocompare::CryptoCompareServiceBuilderOpts;
use bothan_htx::HtxServiceBuilderOpts;

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CryptoSourceConfig {
    pub binance: BinanceServiceBuilderOpts,
    pub coingecko: CoinGeckoServiceBuilderOpts,
    pub coinmarketcap: CoinMarketCapServiceBuilderOpts,
    pub cryptocompare: CryptoCompareServiceBuilderOpts,
    pub htx: HtxServiceBuilderOpts,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RegistryConfig {
    pub source: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub grpc: GrpcConfig,
    pub source: CryptoSourceConfig,
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
