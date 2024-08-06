use std::fs::{create_dir_all, File};
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use reqwest::header::{HeaderName, HeaderValue};
use semver::VersionReq;
use tonic::transport::Server;

use bothan_api::api::CryptoQueryServer;
use bothan_api::config::ipfs::IpfsAuthentication;
use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
use bothan_api::config::AppConfig;
use bothan_api::proto::query::query_server::QueryServer;
use bothan_api::VERSION;
use bothan_binance::BinanceWorkerBuilder;
use bothan_coingecko::CoinGeckoWorkerBuilder;
use bothan_core::ipfs::{IpfsClient, IpfsClientBuilder};
use bothan_core::manager::CryptoAssetInfoManager;
use bothan_core::registry::Registry;
use bothan_core::store::Store;
use bothan_core::worker::AssetWorkerBuilder;

use crate::commands::CliExec;

#[derive(Parser)]
pub struct StartCli {
    /// The config file to use with bothan
    #[arg(long)]
    config: Option<PathBuf>,

    /// A flag to choose whether to start bothan as a fresh instance or not
    #[arg(short, long)]
    reset: bool,

    /// An optional registry file to use on startup. If not provided, an empty registry will be created.
    #[arg(long)]
    registry: Option<PathBuf>,
}

#[async_trait::async_trait]
impl CliExec for StartCli {
    async fn run(&self) {
        let app_config = match &self.config {
            Some(p) => AppConfig::from(p),
            None => AppConfig::with_name("config"),
        }
        .expect("Failed to load config");

        let registry = match &self.registry {
            Some(p) => {
                let mut file = File::open(p).expect("Failed to open registry file");
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)
                    .expect("Failed to read registry file");
                serde_json::from_str(&buffer).expect("Failed to parse registry file")
            }
            None => Registry::default(),
        };

        if let Err(e) = start_server(app_config, registry, self.reset).await {
            println!("Failed to start server: {}", e);
        }
    }
}

async fn start_server(
    app_config: AppConfig,
    registry: Registry,
    reset: bool,
) -> anyhow::Result<()> {
    let log_level = &app_config.log.level;
    tracing_subscriber::fmt()
        .with_env_filter(format!("bothan_core={log_level},bothan_api={log_level}"))
        .init();

    let store = init_store(&app_config, registry, reset).await?;
    let ipfs_client = init_ipfs_client(&app_config).await?;
    let crypto_server = init_crypto_server(&app_config, store, ipfs_client).await?;

    Server::builder()
        .add_service(QueryServer::new(crypto_server))
        .serve(app_config.grpc.addr)
        .await?;

    Ok(())
}

async fn init_store(config: &AppConfig, registry: Registry, reset: bool) -> anyhow::Result<Store> {
    if !config.store.path.is_dir() {
        create_dir_all(&config.store.path).with_context(|| "Failed to create home directory")?;
    }

    let mut store = Store::new(registry, &config.store.path)
        .await
        .with_context(|| "Failed to create store")?;

    if !reset {
        store
            .restore()
            .await
            .with_context(|| "Failed to restore store state")?;
    }

    Ok(store)
}

async fn init_ipfs_client(config: &AppConfig) -> anyhow::Result<IpfsClient> {
    let ipfs_builder = IpfsClientBuilder::new(config.ipfs.endpoint.clone());
    let ipfs_client = match &config.ipfs.authentication {
        IpfsAuthentication::Header { key, value } => {
            let header_name = HeaderName::from_str(key)?;
            let header_value = HeaderValue::from_str(value)?;
            ipfs_builder
                .with_header(header_name, header_value)
                .build()?
        }
        IpfsAuthentication::None => ipfs_builder.build()?,
    };

    Ok(ipfs_client)
}

async fn init_crypto_server(
    config: &AppConfig,
    store: Store,
    ipfs_client: IpfsClient,
) -> anyhow::Result<CryptoQueryServer> {
    let manager_store = Store::create_manager_store(&store);

    let stale_threshold = config.manager.crypto.stale_threshold;
    let version_req = VersionReq::from_str(&format!("<={}", VERSION))
        .with_context(|| "Failed to parse bothan version requirement")?;
    let mut manager =
        CryptoAssetInfoManager::new(manager_store, ipfs_client, stale_threshold, version_req);

    init_crypto_workers(&mut manager, &store, &config.manager.crypto.source).await?;

    Ok(CryptoQueryServer::new(manager))
}

async fn init_crypto_workers(
    manager: &mut CryptoAssetInfoManager<'static>,
    store: &Store,
    source: &CryptoSourceConfigs,
) -> anyhow::Result<()> {
    type Binance = BinanceWorkerBuilder;
    type CoinGecko = CoinGeckoWorkerBuilder;

    add_worker::<Binance>(manager, store, "binance", source.binance.clone()).await?;
    add_worker::<CoinGecko>(manager, store, "coingecko", source.coingecko.clone()).await?;
    Ok(())
}

async fn add_worker<B>(
    manager: &mut CryptoAssetInfoManager<'static>,
    store: &Store,
    worker_name: &str,
    opts: B::Opts,
) -> anyhow::Result<()>
where
    B: AssetWorkerBuilder<'static>,
    B::Error: Send + Sync + 'static,
{
    let worker_store = Store::create_worker_store(store, worker_name);
    let worker = B::new(worker_store, opts)
        .build()
        .await
        .with_context(|| format!("Failed to build {} worker", worker_name))?;

    manager.add_worker(worker_name.to_string(), worker).await;
    Ok(())
}
