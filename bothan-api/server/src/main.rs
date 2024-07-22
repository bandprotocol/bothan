use std::fs::File;
use std::sync::Arc;

use anyhow::ensure;
use tokio::sync::RwLock;
use tonic::transport::Server;
use tracing::info;

use bothan_api::api::CryptoQueryServer;
use bothan_api::config::manager::CryptoSources;
use bothan_api::config::registry::RegistrySource;
use bothan_api::config::AppConfig;
use bothan_api::manager::CryptoAssetInfoManager;
use bothan_api::proto::query::query_server::QueryServer;
use bothan_api::registry::{Registry, Validator};
use bothan_api::utils::add_worker;
use bothan_binance::BinanceWorkerBuilder;
use bothan_coingecko::CoinGeckoWorkerBuilder;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let config = AppConfig::new().expect("Failed to load configuration");
    tracing_subscriber::fmt()
        .with_env_filter(format!("bothan_api={}", config.log.level))
        .init();

    let crypto_query_server = match init_crypto_server(&config).await {
        Ok(server) => server,
        Err(e) => {
            panic!("Failed to initialize crypto server: {:?}", e);
        }
    };

    let addr = config.grpc.addr.clone().parse().unwrap();

    info!("Server running on {}", addr);
    let _ = Server::builder()
        .add_service(QueryServer::new(crypto_query_server))
        .serve(addr)
        .await;
}

async fn init_crypto_server(config: &AppConfig) -> anyhow::Result<CryptoQueryServer> {
    let registry = match &config.manager.crypto.registry {
        RegistrySource::Local(local) => {
            let file = File::open(&local.path)?;
            serde_json::from_reader::<_, Registry>(file)?
        }
        RegistrySource::Ipfs(_) => {
            // TODO: implement ipfs
            todo!()
        }
    };

    ensure!(registry.validate(), "invalid registry".to_string());

    let mut manager = CryptoAssetInfoManager::new(
        Arc::new(RwLock::new(registry)),
        config.manager.crypto.stale_threshold,
    );

    init_crypto_workers(&config.manager.crypto.source, &mut manager).await;

    Ok(CryptoQueryServer::new(manager, None))
}

async fn init_crypto_workers(source: &CryptoSources, manager: &mut CryptoAssetInfoManager) {
    add_worker!(manager, BinanceWorkerBuilder, source.binance);
    add_worker!(manager, CoinGeckoWorkerBuilder, source.coingecko);

    // TODO: reimplement other workers
    // add_worker!(manager, BybitWorkerBuilder, config.source.bybit);
    // add_worker!(manager, CoinbaseWorkerBuilder, config.source.coinbase);
    // add_worker!(manager, CoinMarketCapWorkerBuilder, config.source.coinmarketcap);
    // add_worker!(manager, CryptoCompareWorkerBuilder, config.source.cryptocompare);
    // add_worker!(manager, HtxWorkerBuilder, config.source.htx);
    // add_worker!(manager, KrakenWorkerBuilder, config.source.kraken);
    // add_worker!(manager, OkxWorkerBuilder, config.source.okx);
}
