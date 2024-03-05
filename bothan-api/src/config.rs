use config::Config;
use serde::Deserialize;
use tokio::time::Duration;

#[derive(Debug, Deserialize)]
pub struct GrpcConfig {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct BinanceServiceConfig {
    pub url: Option<String>,
    pub cmd_ch_size: Option<usize>,
    pub rem_id_ch_size: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct CoingeckoServiceConfig {
    pub url: Option<String>,
    pub api_key: Option<String>,
    pub user_agent: Option<String>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_interval: Option<Duration>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub update_supported_assets_interval: Option<Duration>,
    pub page_size: Option<usize>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub page_query_delay: Option<Duration>,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub binance: BinanceServiceConfig,
    pub coingecko: CoingeckoServiceConfig,
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
