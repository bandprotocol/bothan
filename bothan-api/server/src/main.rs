use std::sync::Arc;

use tonic::transport::Server;
use tracing::info;

use bothan_api::api::CryptoQueryServer;
use bothan_api::config::manager::crypto_info::registry::RegistrySeedConfig;
use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
use bothan_api::config::AppConfig;
use bothan_api::manager::CryptoAssetInfoManager;
use bothan_api::proto::query::query_server::QueryServer;
use bothan_api::registry::Registry;
use bothan_api::utils::add_worker;
use bothan_binance::BinanceWorkerBuilder;
use bothan_coingecko::CoinGeckoWorkerBuilder;

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
    let ipfs_config = config.ipfs.clone();

    let seed_registry = match &config.manager.crypto.registry {
        RegistrySeedConfig::Local { path } => Registry::try_from_path(path)?,
        RegistrySeedConfig::Ipfs { hash } => {
            Registry::try_from_ipfs(&ipfs_config.endpoint, hash, &ipfs_config.authentication)
                .await?
        }
    };

    let stale_threshold = config.manager.crypto.stale_threshold;
    let mut manager = CryptoAssetInfoManager::new(stale_threshold, seed_registry, ipfs_config);

    let source_config = &config.manager.crypto.source;
    init_crypto_workers(source_config, &mut manager).await;

    Ok(CryptoQueryServer::new(manager))
}

async fn init_crypto_workers(source: &CryptoSourceConfigs, manager: &mut CryptoAssetInfoManager) {
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
