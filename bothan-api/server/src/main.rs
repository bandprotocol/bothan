use std::fs::File;
use std::sync::Arc;

use tonic::transport::Server;
use tracing::info;

use bothan_binance::BinanceServiceBuilder;
use bothan_bybit::BybitServiceBuilder;
use bothan_coingecko::CoinGeckoServiceBuilder;
use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
use bothan_cryptocompare::CryptoCompareServiceBuilder;
use bothan_htx::HtxServiceBuilder;
use bothan_kraken::KrakenServiceBuilder;

use crate::api::CryptoQueryServer;
use crate::config::AppConfig;
use crate::manager::price_service::manager::PriceServiceManager;
use crate::proto::query::query_server::QueryServer;
use crate::registry::Registry;
use crate::utils::add_service;

mod api;
mod config;
mod manager;
mod post_processor;
mod processor;
mod proto;
mod registry;
mod tasks;
mod utils;

#[tokio::main]
async fn main() {
    let config = AppConfig::new().expect("Failed to load configuration");
    tracing_subscriber::fmt()
        .with_env_filter(format!("bothan_api={}", config.logging.level))
        .init();

    let crypto_query_server = init_crypto_server(&config).await;

    let addr = config.grpc.addr.clone().parse().unwrap();

    info!("Server running on {}", addr);
    let _ = Server::builder()
        .add_service(QueryServer::new(crypto_query_server))
        .serve(addr)
        .await;
}

async fn init_crypto_server(config: &AppConfig) -> CryptoQueryServer {
    let file = File::open(config.registry.crypto_price.source.clone()).unwrap();
    let registry = Arc::new(serde_json::from_reader::<_, Registry>(file).unwrap());
    let mut manager = PriceServiceManager::new(registry, config.manager.stale_threshold)
        .expect("cannot build price service manager with registry");

    init_crypto_services(config, &mut manager).await;

    CryptoQueryServer::new(manager)
}

async fn init_crypto_services(config: &AppConfig, manager: &mut PriceServiceManager) {
    add_service!(manager, BinanceServiceBuilder, config.source.binance);
    add_service!(manager, BybitServiceBuilder, config.source.bybit);
    add_service!(manager, CoinGeckoServiceBuilder, config.source.coingecko);
    add_service!(
        manager,
        CoinMarketCapServiceBuilder,
        config.source.coinmarketcap
    );
    add_service!(
        manager,
        CryptoCompareServiceBuilder,
        config.source.cryptocompare
    );
    add_service!(manager, HtxServiceBuilder, config.source.htx);
    add_service!(manager, KrakenServiceBuilder, config.source.kraken);
}
