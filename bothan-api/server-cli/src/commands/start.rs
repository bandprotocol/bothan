use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, remove_dir_all, write, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use reqwest::header::{HeaderName, HeaderValue};
use semver::{Version, VersionReq};
use tonic::transport::Server;
use tracing::{debug, error, info};

use bothan_api::api::BothanServer;
use bothan_api::config::ipfs::IpfsAuthentication;
use bothan_api::config::manager::crypto_info::sources::CryptoSourceConfigs;
use bothan_api::config::AppConfig;
use bothan_api::proto::bothan::v1::BothanServiceServer;
use bothan_api::{REGISTRY_REQUIREMENT, VERSION};
use bothan_binance::BinanceWorkerBuilder;
use bothan_bybit::BybitWorkerBuilder;
use bothan_coinbase::CoinbaseWorkerBuilder;
use bothan_coingecko::CoinGeckoWorkerBuilder;
use bothan_coinmarketcap::CoinMarketCapWorkerBuilder;
use bothan_core::ipfs::{IpfsClient, IpfsClientBuilder};
use bothan_core::manager::CryptoAssetInfoManager;
use bothan_core::monitoring::{Client as MonitoringClient, Signer};
use bothan_core::registry::{Registry, Valid};
use bothan_core::store::SharedStore;
use bothan_core::worker::{AssetWorker, AssetWorkerBuilder};
use bothan_cryptocompare::CryptoCompareWorkerBuilder;
use bothan_htx::HtxWorkerBuilder;
use bothan_kraken::KrakenWorkerBuilder;
use bothan_okx::OkxWorkerBuilder;

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
                serde_json::from_reader(reader).with_context(|| "Failed to parse registry")?
            }
            None => Registry::default(),
        };

        let valid_registry = registry
            .validate()
            .with_context(|| "Failed to validate registry")?;

        let store = init_store(&app_config, valid_registry, self.unsafe_reset).await?;
        let ipfs_client = init_ipfs_client(&app_config).await?;
        let monitoring_client = init_monitoring_client(&app_config).await?;

        let bothan_server =
            init_bothan_server(&app_config, store, ipfs_client, monitoring_client).await?;

        info!("server started");
        Server::builder()
            .add_service(BothanServiceServer::from_arc(bothan_server))
            .serve(app_config.grpc.addr)
            .await?;

        Ok(())
    }
}

async fn init_store(
    config: &AppConfig,
    registry: Registry<Valid>,
    reset: bool,
) -> anyhow::Result<SharedStore> {
    if reset {
        remove_dir_all(&config.store.path).with_context(|| "Failed to remove store directory")?;
    }

    if !config.store.path.is_dir() {
        create_dir_all(&config.store.path).with_context(|| "Failed to create home directory")?;
    }

    let mut store = SharedStore::new(registry, &config.store.path)
        .await
        .with_context(|| "Failed to create store")?;

    debug!("store created successfully at {:?}", &config.store.path);

    if !reset {
        store
            .restore()
            .await
            .with_context(|| "Failed to restore store state")?;
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

async fn init_bothan_server(
    config: &AppConfig,
    store: SharedStore,
    ipfs_client: IpfsClient,
    monitoring_client: Option<Arc<MonitoringClient>>,
) -> anyhow::Result<Arc<BothanServer>> {
    let manager_store = SharedStore::create_manager_store(&store);

    let stale_threshold = config.manager.crypto.stale_threshold;
    let bothan_version =
        Version::from_str(VERSION).with_context(|| "Failed to parse bothan version")?;
    let registry_version_requirement = VersionReq::from_str(REGISTRY_REQUIREMENT)
        .with_context(|| "Failed to parse registry version requirement")?;

    let workers = init_crypto_workers(&store, &config.manager.crypto.source).await?;
    let manager = CryptoAssetInfoManager::new(
        workers,
        manager_store,
        ipfs_client,
        stale_threshold,
        bothan_version,
        registry_version_requirement,
        monitoring_client,
    );

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

async fn init_crypto_workers(
    store: &SharedStore,
    source: &CryptoSourceConfigs,
) -> anyhow::Result<HashMap<String, Arc<dyn AssetWorker>>> {
    type Binance = BinanceWorkerBuilder;
    type Bybit = BybitWorkerBuilder;
    type Coinbase = CoinbaseWorkerBuilder;
    type CoinGecko = CoinGeckoWorkerBuilder;
    type CoinMarketCap = CoinMarketCapWorkerBuilder;
    type CryptoCompare = CryptoCompareWorkerBuilder;
    type Htx = HtxWorkerBuilder;
    type Kraken = KrakenWorkerBuilder;
    type Okx = OkxWorkerBuilder;

    let mut workers = HashMap::new();

    if let Some(opts) = &source.binance {
        add_worker::<Binance>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.bybit {
        add_worker::<Bybit>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.coinbase {
        add_worker::<Coinbase>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.coingecko {
        add_worker::<CoinGecko>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.coinmarketcap {
        add_worker::<CoinMarketCap>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.cryptocompare {
        add_worker::<CryptoCompare>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.htx {
        add_worker::<Htx>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.kraken {
        add_worker::<Kraken>(&mut workers, store, opts).await?;
    }

    if let Some(opts) = &source.okx {
        add_worker::<Okx>(&mut workers, store, opts).await?;
    }

    Ok(workers)
}

async fn add_worker<B>(
    workers: &mut HashMap<String, Arc<dyn AssetWorker>>,
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

    workers.insert(worker_name.to_string(), worker);
    info!("loaded {} worker", worker_name);
    Ok(())
}
