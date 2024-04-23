use std::fs::File;
use std::sync::Arc;

use tonic::transport::Server;

use bothan_binance::BinanceServiceBuilder;
use bothan_coingecko::CoinGeckoServiceBuilder;
use bothan_coinmarketcap::CoinMarketCapServiceBuilder;
use bothan_cryptocompare::CryptoCompareServiceBuilder;
use bothan_htx::HtxServiceBuilder;

use crate::api::APIServiceImpl;
use crate::config::AppConfig;
use crate::manager::price_service::manager::PriceServiceManager;
use crate::proto::query::query::query_server::QueryServer;
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

    let file = File::open(config.registry.source.clone()).unwrap();
    let registry = Arc::new(serde_json::from_reader::<_, Registry>(file).unwrap());
    let mut manager = PriceServiceManager::new(registry)
        .expect("cannot build price service manager with registry");
    let addr = config.grpc.addr.clone().parse().unwrap();

    initialize_services(config, &mut manager).await;
    let api_service_impl = APIServiceImpl::new(manager);
    println!("Server running on {}", addr);

    let _ = Server::builder()
        .add_service(QueryServer::new(api_service_impl))
        .serve(addr)
        .await;
}

async fn initialize_services(config: AppConfig, manager: &mut PriceServiceManager) {
    add_service!(manager, BinanceServiceBuilder, config.source.binance);
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
}
