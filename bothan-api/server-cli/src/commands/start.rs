use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use clap::Parser;
use reqwest::header::{HeaderName, HeaderValue};
use semver::VersionReq;
use tonic::transport::Server;

use crate::commands::utils::bothan_home_dir;
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
use bothan_core::registry::{Registry, Valid};
use bothan_core::store::SharedStore;
use bothan_core::worker::AssetWorkerBuilder;
use bothan_kraken::KrakenWorkerBuilder;

#[derive(Parser)]
pub struct StartCli {
    /// The configuration file to use with bothan
    #[arg(long)]
    config: Option<PathBuf>,

    /// A flag to choose whether to start bothan as a fresh instance or not
    #[arg(short, long)]
    unsafe_reset: bool,

    /// Flag to turn on dev mode
    #[arg(short, long)]
    dev: bool,

    /// An optional initial registry state to use on startup to be used for testing purposes only.
    #[arg(long, requires = "dev")]
    registry: Option<PathBuf>,
}

impl StartCli {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config_path = match &self.config {
            Some(p) => p.clone(),
            None => bothan_home_dir().join("config.toml"),
        };

        let app_config = AppConfig::from(config_path)?;

        let registry = match &self.registry {
            Some(p) => {
                let file =
                    File::open(p).map_err(|e| anyhow!("Failed to open registry file: {e}"))?;
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)
                    .map_err(|e| anyhow!("Failed to parse registry file: {e}"))?
            }
            None => Registry::default(),
        };

        let valid_registry = registry
            .validate()
            .map_err(|e| anyhow!("Invalid registry: {e}"))?;

        if let Err(e) = start_server(app_config, valid_registry, self.unsafe_reset).await {
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
        Ok(())
    }
}

async fn start_server(
    app_config: AppConfig,
    registry: Registry<Valid>,
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

async fn init_store(
    config: &AppConfig,
    registry: Registry<Valid>,
    reset: bool,
) -> anyhow::Result<SharedStore> {
    if reset {
        if let Err(e) = std::fs::remove_dir_all(&config.store.path) {
            eprintln!("Failed to remove store directory: {}", e);
        }
    }

    if !config.store.path.is_dir() {
        create_dir_all(&config.store.path).with_context(|| "Failed to create home directory")?;
    }

    let mut store = SharedStore::new(registry, &config.store.path)
        .await
        .with_context(|| "Failed to create store")?;
    println!("Store created successfully at \"{:?}\"", &config.store.path);

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
    store: SharedStore,
    ipfs_client: IpfsClient,
) -> anyhow::Result<CryptoQueryServer> {
    let manager_store = SharedStore::create_manager_store(&store);

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
    store: &SharedStore,
    source: &CryptoSourceConfigs,
) -> anyhow::Result<()> {
    type Binance = BinanceWorkerBuilder;
    type CoinGecko = CoinGeckoWorkerBuilder;
    type Kraken = KrakenWorkerBuilder;

    if let Some(opts) = &source.binance {
        add_worker::<Binance>(manager, store, opts).await?;
    }

    if let Some(opts) = &source.coingecko {
        add_worker::<CoinGecko>(manager, store, opts).await?;
    }

    if let Some(opts) = &source.kraken {
        add_worker::<Kraken>(manager, store, opts).await?;
    }

    Ok(())
}

async fn add_worker<B>(
    manager: &mut CryptoAssetInfoManager<'static>,
    store: &SharedStore,
    opts: &B::Opts,
) -> anyhow::Result<()>
where
    B: AssetWorkerBuilder<'static>,
    B::Error: Send + Sync + 'static,
    B::Opts: Clone,
{
    let worker_name = B::worker_name();
    let worker_store = SharedStore::create_worker_store(store, worker_name);
    let worker = B::new(worker_store, opts.clone())
        .build()
        .await
        .with_context(|| format!("Failed to build worker {worker_name}"))?;

    manager.add_worker(worker_name.to_string(), worker).await;
    println!("Loaded {} worker", worker_name);
    Ok(())
}
