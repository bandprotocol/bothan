use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use semver::Version;
use tokio::sync::RwLock;

use bothan_core::worker::AssetWorker;

use crate::config::ipfs::IpfsConfig;
use crate::manager::crypto_asset_info::error::SetRegistryError;
use crate::manager::crypto_asset_info::price::get_prices;
use crate::manager::crypto_asset_info::signal_ids::{
    add_worker_query_ids, remove_worker_query_ids,
};
use crate::manager::crypto_asset_info::utils::valid_version;
use crate::proto::query::Price;
use crate::registry::Registry;

pub struct CryptoAssetInfoManager {
    workers: RwLock<HashMap<String, Arc<dyn AssetWorker>>>,
    active_signal_ids: Arc<RwLock<HashSet<String>>>,
    stale_threshold: i64,
    registry: Arc<RwLock<Registry>>,
    ipfs_config: IpfsConfig,
}

impl CryptoAssetInfoManager {
    pub fn new(stale_threshold: i64, registry: Registry, ipfs_config: IpfsConfig) -> Self {
        CryptoAssetInfoManager {
            workers: RwLock::new(HashMap::new()),
            active_signal_ids: Arc::new(RwLock::new(HashSet::new())),
            stale_threshold,
            registry: Arc::new(RwLock::new(registry)),
            ipfs_config,
        }
    }

    /// Adds a worker with an assigned name.
    pub async fn add_worker(&mut self, name: String, worker: Arc<dyn AssetWorker>) {
        self.workers.write().await.insert(name, worker);
    }

    /// Sets the active signal ids of the manager.
    pub async fn set_active_signal_ids(&mut self, signal_ids: Vec<String>) -> anyhow::Result<()> {
        let active_signal_ids = self.active_signal_ids.write().await;
        let new_active_set = signal_ids.into_iter().collect::<HashSet<String>>();
        let registry = self.registry.read().await;
        let workers = self.workers.write().await;

        add_worker_query_ids(&workers, &active_signal_ids, &new_active_set, &registry).await?;
        remove_worker_query_ids(&workers, &active_signal_ids, &new_active_set, &registry).await?;
        Ok(())
    }

    /// Gets the `Price` of the given signal ids.
    pub async fn get_prices(&mut self, ids: Vec<String>) -> anyhow::Result<Vec<Price>> {
        let registry = self.registry.read().await;
        let workers = self.workers.write().await;
        get_prices(ids, &registry, &workers, self.stale_threshold).await
    }

    pub async fn set_registry(
        &mut self,
        hash: &str,
        version: Version,
    ) -> Result<(), SetRegistryError> {
        if !valid_version(version) {
            return Err(SetRegistryError::UnsupportedVersion);
        }

        let config = &self.ipfs_config;
        let new_registry =
            Registry::try_from_ipfs(&config.endpoint, hash, &config.authentication).await;

        let reg = match new_registry {
            Ok(reg) => reg,
            Err(_) => return Err(SetRegistryError::InvalidRegistry),
        };

        if !reg.is_valid() {
            return Err(SetRegistryError::InvalidRegistry);
        }

        let mut registry = self.registry.write().await;
        *registry = reg;
        Ok(())
    }
}
