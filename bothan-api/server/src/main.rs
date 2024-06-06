use std::fs::File;
use std::sync::Arc;

use anyhow::{bail, Result};
use tonic::transport::Server;
use tracing::info;

use bothan_api::api::CryptoQueryServer;
use bothan_api::config::AppConfig;
use bothan_api::manager::PriceServiceManager;
use bothan_api::proto::query::query_server::QueryServer;
use bothan_api::registry::{Registry, Validator};
use bothan_api::utils::add_service;
use bothan_binance::BinanceServiceBuilder;
use bothan_bybit::BybitServiceBuilder;
use bothan_coinbase::CoinbaseServiceBuilder;
use bothan_coingecko::CoinGeckoServiceBuilder;
use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
use bothan_cryptocompare::CryptoCompareServiceBuilder;
use bothan_htx::HtxServiceBuilder;
use bothan_kraken::KrakenServiceBuilder;
use bothan_okx::OkxServiceBuilder;

#[tokio::main]
async fn main() {
    let config = AppConfig::new().expect("Failed to load configuration");
    tracing_subscriber::fmt()
        .with_env_filter(format!("bothan_api={}", config.logging.level))
        .init();

    let crypto_query_server = init_crypto_server(&config)
        .await
        .expect("cannot initialize crypto server");

    let addr = config.grpc.addr.clone().parse().unwrap();

    info!("Server running on {}", addr);
    let _ = Server::builder()
        .add_service(QueryServer::new(crypto_query_server))
        .serve(addr)
        .await;
}

async fn init_crypto_server(config: &AppConfig) -> Result<CryptoQueryServer> {
    let file = File::open(config.registry.crypto_price.source.clone())?;
    let registry = Arc::new(serde_json::from_reader::<_, Registry>(file)?);
    if !registry.validate() {
        bail!("registry validation failed".to_string());
    }
    let mut manager = PriceServiceManager::new(registry, config.manager.stale_threshold)
        .expect("cannot build price service manager with registry");

    init_crypto_services(config, &mut manager).await;

    Ok(CryptoQueryServer::new(manager))
}

#[rustfmt::skip]
async fn init_crypto_services(config: &AppConfig, manager: &mut PriceServiceManager) {
    add_service!(manager, BinanceServiceBuilder, config.source.binance);
    add_service!(manager, BybitServiceBuilder, config.source.bybit);
    add_service!(manager, CoinbaseServiceBuilder, config.source.coinbase);
    add_service!(manager, CoinGeckoServiceBuilder, config.source.coingecko);
    add_service!(manager, CoinMarketCapServiceBuilder, config.source.coinmarketcap);
    add_service!(manager, CryptoCompareServiceBuilder, config.source.cryptocompare);
    add_service!(manager, HtxServiceBuilder, config.source.htx);
    add_service!(manager, KrakenServiceBuilder, config.source.kraken);
    add_service!(manager, OkxServiceBuilder, config.source.okx);
}
