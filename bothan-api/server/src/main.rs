use std::path::PathBuf;
use std::str::FromStr;

use reqwest::header::{HeaderName, HeaderValue};
use semver::VersionReq;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncReadExt;
use tonic::transport::Server;
use tracing::info;

use bothan_api::api::CryptoQueryServer;
use bothan_api::config::ipfs::IpfsAuthentication;
use bothan_api::config::manager::crypto_info::registry::RegistrySeedConfig;
use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
use bothan_api::config::AppConfig;
use bothan_api::proto::query::query_server::QueryServer;
use bothan_api::VERSION;
use bothan_binance::BinanceWorkerBuilder;
use bothan_coingecko::CoinGeckoWorkerBuilder;
use bothan_core::ipfs::IpfsClientBuilder;
use bothan_core::manager::CryptoAssetInfoManager;
use bothan_core::registry::Registry;
use bothan_core::store::Store;
use bothan_core::worker::AssetWorkerBuilder;

#[tokio::main]
async fn main() {
    let config = AppConfig::new().expect("Failed to load configuration");
    let log_level = &config.log.level;
    tracing_subscriber::fmt()
        .with_env_filter(format!("bothan_core={log_level},bothan_api={log_level}"))
        .init();

    // Create root directory
    let root = PathBuf::from_str(&config.store.path).expect("Failed to parse store path");

    if !root.is_dir() {
        create_dir_all(&root)
            .await
            .expect("Failed to create root directory");
    }

    let crypto_query_server = init_crypto_server(&config, root)
        .await
        .expect("Failed to initialize crypto server");

    let addr = config
        .grpc
        .addr
        .parse()
        .expect("Failed to parse GRPC address");

    info!("Server running on {}", addr);
    let _ = Server::builder()
        .add_service(QueryServer::new(crypto_query_server))
        .serve(addr)
        .await;
}

async fn init_crypto_server(
    config: &AppConfig,
    save_dir: PathBuf,
) -> anyhow::Result<CryptoQueryServer> {
    let ipfs_builder = IpfsClientBuilder::new(config.ipfs.endpoint.clone());
    let ipfs_builder = match &config.ipfs.authentication {
        IpfsAuthentication::Header { key, value } => {
            let header_name = HeaderName::from_str(key).expect("Failed to parse header key");
            let header_value = HeaderValue::from_str(value).expect("Failed to parse header value");
            ipfs_builder.with_header(header_name, header_value)
        }
        IpfsAuthentication::None => ipfs_builder,
    };

    let ipfs_client = ipfs_builder.build().expect("Failed to build IPFS client");

    let seed_registry: Registry = match &config.manager.crypto.registry {
        RegistrySeedConfig::Local { path } => {
            let mut file = File::open(path)
                .await
                .expect("Failed to open registry file");
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).await?;
            serde_json::from_str(&buffer).expect("Failed to parse registry file")
        }
        RegistrySeedConfig::Ipfs { hash } => {
            let ipfs = ipfs_client
                .get_ipfs(hash)
                .await
                .expect("Failed to get IPFS registry");
            serde_json::from_str(&ipfs).expect("Failed to parse IPFS registry")
        }
    };

    let mut store = Store::new(seed_registry, save_dir.as_path())
        .await
        .expect("Failed to create store");

    match store.restore().await {
        Ok(_) => info!("Store restored successfully"),
        Err(e) => info!("Failed to restore store: {}", e),
    }

    let manager_store = Store::create_manager_store(&store);

    let stale_threshold = config.manager.crypto.stale_threshold;
    let version_req = VersionReq::from_str(&format!("<={}", VERSION))
        .expect("Failed to parse bothan version requirement");
    let mut manager =
        CryptoAssetInfoManager::new(manager_store, ipfs_client, stale_threshold, version_req);

    init_crypto_workers(&mut manager, &store, &config.manager.crypto.source).await;

    Ok(CryptoQueryServer::new(manager))
}

async fn init_crypto_workers(
    manager: &mut CryptoAssetInfoManager<'static>,
    store: &Store,
    source: &CryptoSourceConfigs,
) {
    type Binance = BinanceWorkerBuilder;
    type CoinGecko = CoinGeckoWorkerBuilder;

    add_worker::<Binance>(manager, store, "binance", source.binance.clone()).await;
    add_worker::<CoinGecko>(manager, store, "coingecko", source.coingecko.clone()).await;

    // TODO: reimplement other workers
    // add_worker!(manager, store.clone(), BybitWorkerBuilder, source.bybit);
    // add_worker!(manager, store.clone(), CoinbaseWorkerBuilder, source.coinbase);
    // add_worker!(manager, store.clone(), CoinMarketCapWorkerBuilder, source.coinmarketcap);
    // add_worker!(manager, store.clone(), CryptoCompareWorkerBuilder, source.cryptocompare);
    // add_worker!(manager, store.clone(), HtxWorkerBuilder, source.htx);
    // add_worker!(manager, store.clone(), KrakenWorkerBuilder, source.kraken);
    // add_worker!(manager, store.clone(), OkxWorkerBuilder, source.okx);
}

async fn add_worker<B>(
    manager: &mut CryptoAssetInfoManager<'static>,
    store: &Store,
    worker_name: &str,
    opts: B::Opts,
) where
    B: AssetWorkerBuilder<'static>,
{
    let worker_store = Store::create_worker_store(store, worker_name);
    let worker = B::new(worker_store, opts)
        .build()
        .await
        .expect("Failed to build worker");

    manager.add_worker(worker_name.to_string(), worker).await;
}
