use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::RwLock;

use bothan_core::worker::AssetWorker;

use crate::manager::crypto_asset_info::price::get_prices;
use crate::manager::crypto_asset_info::signal_ids::{
    add_worker_query_ids, remove_worker_query_ids,
};
use crate::proto::query::Price;
use crate::registry::Registry;

pub struct CryptoAssetInfoManager {
    workers: RwLock<HashMap<String, Arc<dyn AssetWorker>>>,
    active_signal_ids: Arc<RwLock<HashSet<String>>>,
    registry: Arc<RwLock<Registry>>,
    stale_threshold: i64,
}

impl CryptoAssetInfoManager {
    /// Creates a new `CryptoAssetInfoManager` from the given registry and stale threshold.
    pub fn new(registry: Arc<RwLock<Registry>>, stale_threshold: i64) -> Self {
        CryptoAssetInfoManager {
            workers: RwLock::new(HashMap::new()),
            active_signal_ids: Arc::new(RwLock::new(HashSet::new())),
            registry,
            stale_threshold,
        }
    }

    /// Adds a worker with an assigned name.
    pub async fn add_worker(&mut self, name: String, worker: Arc<dyn AssetWorker>) {
        self.workers.write().await.insert(name, worker);
    }

    /// Sets the registry of the manager.
    pub async fn set_registry(&mut self, registry: Registry) {
        let mut writer = self.registry.write().await;
        *writer = registry;
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
}
