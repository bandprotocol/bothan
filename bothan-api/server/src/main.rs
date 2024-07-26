use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use dirs::home_dir;
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
use bothan_api::utils::add_worker;
use bothan_api::VERSION;
use bothan_binance::BinanceWorkerBuilder;
use bothan_coingecko::CoinGeckoWorkerBuilder;
use bothan_core::ipfs::IpfsClientBuilder;
use bothan_core::manager::CryptoAssetInfoManager;
use bothan_core::registry::Registry;
use bothan_core::store::{ManagerStore, Store};

#[tokio::main]
async fn main() {
    let config = AppConfig::new().expect("Failed to load configuration");
    let log_level = &config.log.level;
    tracing_subscriber::fmt()
        .with_env_filter(format!("bothan_core={log_level},bothan_api={log_level}"))
        .init();

    // Create root directory
    let root = home_dir()
        .expect("Failed to get home directory")
        .join(config.store.path.as_str());

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
    root: PathBuf,
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

    let store = Arc::new(
        Store::new(seed_registry, root.as_path())
            .await
            .expect("Failed to create store"),
    );
    let manager_store = ManagerStore::from(store.clone());

    let stale_threshold = config.manager.crypto.stale_threshold;
    let version_req = VersionReq::from_str(&format!("<={}", VERSION))
        .expect("Failed to parse bothan version requirement");
    let mut manager =
        CryptoAssetInfoManager::new(manager_store, ipfs_client, stale_threshold, version_req);

    init_crypto_workers(&mut manager, store, &config.manager.crypto.source).await;

    Ok(CryptoQueryServer::new(manager))
}

#[rustfmt::skip]
async fn init_crypto_workers(
    manager: &mut CryptoAssetInfoManager,
    store: Arc<Store>,
    source: &CryptoSourceConfigs,
) {
    add_worker!(manager, store.clone(), BinanceWorkerBuilder, source.binance);
    add_worker!(manager, store.clone(), CoinGeckoWorkerBuilder, source.coingecko);
    
    // TODO: reimplement other workers
    // add_worker!(manager, store.clone(), BybitWorkerBuilder, source.bybit);
    // add_worker!(manager, store.clone(), CoinbaseWorkerBuilder, source.coinbase);
    // add_worker!(manager, store.clone(), CoinMarketCapWorkerBuilder, source.coinmarketcap);
    // add_worker!(manager, store.clone(), CryptoCompareWorkerBuilder, source.cryptocompare);
    // add_worker!(manager, store.clone(), HtxWorkerBuilder, source.htx);
    // add_worker!(manager, store.clone(), KrakenWorkerBuilder, source.kraken);
    // add_worker!(manager, store.clone(), OkxWorkerBuilder, source.okx);
}
