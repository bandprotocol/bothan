use std::collections::HashMap;
use std::fs::{File, create_dir_all, read_to_string, write};
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, bail};
use bothan_api::api::BothanServer;
use bothan_api::config::AppConfig;
use bothan_api::config::ipfs::IpfsAuthentication;
use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
use bothan_api::proto::bothan::v1::{BothanServiceServer, FILE_DESCRIPTOR_SET};
use bothan_api::{REGISTRY_REQUIREMENT, VERSION};
use bothan_core::ipfs::{IpfsClient, IpfsClientBuilder};
use bothan_core::manager::CryptoAssetInfoManager;
use bothan_core::manager::crypto_asset_info::CryptoAssetWorkerOpts;
use bothan_core::monitoring::{Client as MonitoringClient, Signer};
use bothan_core::store::rocksdb::RocksDbStore;
use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::worker::error::AssetWorkerError;
use clap::Parser;
use reqwest::header::{HeaderName, HeaderValue};
use semver::{Version, VersionReq};
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tracing::{debug, error, info};

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
    pub async fn run(&self, app_config: AppConfig) -> anyhow::Result<()> {
        let registry = match &self.registry {
            Some(p) => {
                let file = File::open(p).with_context(|| "Failed to open registry file")?;
                let reader = BufReader::new(file);
                let registry =
                    serde_json::from_reader(reader).with_context(|| "Failed to parse registry")?;
                Some(registry)
            }
            None => None,
        };

        let store = init_rocks_db_store(&app_config, registry, self.unsafe_reset).await?;
        let ipfs_client = init_ipfs_client(&app_config).await?;
        let monitoring_client = init_monitoring_client(&app_config).await?;

        let bothan_server =
            init_bothan_server(&app_config, store, ipfs_client, monitoring_client).await?;

        let reflection_service = ReflectionBuilder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1alpha()
            .with_context(|| "Failed to build reflection service")?;

        info!("server started at {:?}", app_config.grpc.addr);

        Server::builder()
            .add_service(reflection_service)
            .add_service(BothanServiceServer::from_arc(bothan_server))
            .serve(app_config.grpc.addr)
            .await?;

        Ok(())
    }
}

async fn init_rocks_db_store(
    config: &AppConfig,
    registry: Option<Registry<Valid>>,
    reset: bool,
) -> anyhow::Result<RocksDbStore> {
    let flush_path = &config.store.path;

    let store = match (reset, flush_path.is_dir()) {
        // If reset is true and the path is a directory, remove the directory and create a new store
        (true, true) => {
            let db = RocksDbStore::new(flush_path)?;
            debug!("store reset successfully at {:?}", &flush_path);
            db
        }
        // If no reset, load the store
        (false, true) => {
            let db = RocksDbStore::load(flush_path)?;
            debug!("store loaded successfully at {:?}", &flush_path);
            db
        }
        // If the path does not exist, create the directory and create a new store
        (_, false) => {
            create_dir_all(flush_path).with_context(|| "Failed to create home directory")?;
            let db = RocksDbStore::new(flush_path)?;
            debug!("store created successfully at {:?}", &flush_path);
            db
        }
    };

    // If a registry is provided, overwrite the registry
    if let Some(registry) = registry {
        store.set_registry(registry, "".to_string()).await?;
    }

    Ok(store)
}

async fn init_signer(config: &AppConfig) -> anyhow::Result<Signer> {
    if config.monitoring.path.is_file() {
        let pk = read_to_string(&config.monitoring.path)
            .with_context(|| "Failed to read monitoring key file")?;

        Signer::from_hex(&pk).with_context(|| "Failed to parse monitoring key file")
    } else {
        let signer = Signer::random();

        if let Some(parent) = config.monitoring.path.parent() {
            create_dir_all(parent).with_context(|| "Failed to create monitoring key directory")?;
        }

        write(&config.monitoring.path, signer.to_hex().as_bytes())
            .with_context(|| "Failed to write monitoring key file")?;

        Ok(signer)
    }
}

async fn init_monitoring_client(
    config: &AppConfig,
) -> anyhow::Result<Option<Arc<MonitoringClient>>> {
    if !config.monitoring.enabled {
        return Ok(None);
    }

    let signer = init_signer(config)
        .await
        .with_context(|| "Failed to build signer")?;
    let monitoring = Arc::new(MonitoringClient::new(&config.monitoring.endpoint, signer));
    Ok(Some(monitoring))
}

async fn init_ipfs_client(config: &AppConfig) -> anyhow::Result<IpfsClient> {
    let ipfs_builder = IpfsClientBuilder::new(&config.ipfs.endpoint);
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

async fn init_bothan_server<S: Store + 'static>(
    config: &AppConfig,
    store: S,
    ipfs_client: IpfsClient,
    monitoring_client: Option<Arc<MonitoringClient>>,
) -> anyhow::Result<Arc<BothanServer<S>>> {
    let stale_threshold = config.manager.crypto.stale_threshold;
    let bothan_version =
        Version::from_str(VERSION).with_context(|| "Failed to parse bothan version")?;
    let registry_version_requirement = VersionReq::from_str(REGISTRY_REQUIREMENT)
        .with_context(|| "Failed to parse registry version requirement")?;

    let opts = match init_crypto_opts(&config.manager.crypto.source).await {
        Ok(workers) => workers,
        Err(e) => {
            bail!("failed to initialize workers: {:?}", e);
        }
    };
    let manager = match CryptoAssetInfoManager::build(
        store,
        opts,
        ipfs_client,
        stale_threshold,
        bothan_version,
        registry_version_requirement,
        monitoring_client,
    )
    .await
    {
        Ok(manager) => manager,
        Err(e) => {
            bail!("failed to build manager: {:?}", e);
        }
    };

    let manager = Arc::new(manager);
    let cloned_manager = manager.clone();

    // Only spawn heartbeat if monitoring is enabled
    if config.monitoring.enabled {
        tokio::spawn(async move {
            loop {
                // Heartbeat is fixed at 1 minute.
                tokio::time::sleep(Duration::from_secs(60)).await;
                match cloned_manager.post_heartbeat().await {
                    Ok(_) => info!("heartbeat sent"),
                    Err(e) => error!("failed to send heartbeat: {e}"),
                }
            }
        });
    }

    Ok(Arc::new(BothanServer::new(manager)))
}

async fn init_crypto_opts(
    source: &CryptoSourceConfigs,
) -> Result<HashMap<String, CryptoAssetWorkerOpts>, AssetWorkerError> {
    let mut worker_opts = HashMap::new();

    add_worker_opts(&mut worker_opts, &source.binance).await?;
    add_worker_opts(&mut worker_opts, &source.bitfinex).await?;
    add_worker_opts(&mut worker_opts, &source.bybit).await?;
    add_worker_opts(&mut worker_opts, &source.coinbase).await?;
    add_worker_opts(&mut worker_opts, &source.coingecko).await?;
    add_worker_opts(&mut worker_opts, &source.coinmarketcap).await?;
    add_worker_opts(&mut worker_opts, &source.htx).await?;
    add_worker_opts(&mut worker_opts, &source.kraken).await?;
    add_worker_opts(&mut worker_opts, &source.okx).await?;

    Ok(worker_opts)
}

async fn add_worker_opts<O: Clone + Into<CryptoAssetWorkerOpts>>(
    workers_opts: &mut HashMap<String, CryptoAssetWorkerOpts>,
    opts: &Option<O>,
) -> Result<(), AssetWorkerError> {
    if let Some(opts) = opts {
        let worker_opts = opts.clone().into();
        let worker_name = worker_opts.name();
        info!("{} worker is enabled", worker_name);
        workers_opts.insert(worker_name.to_string(), worker_opts);
    }

    Ok(())
}
