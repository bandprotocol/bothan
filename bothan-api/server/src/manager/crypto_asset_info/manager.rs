use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use log::{error, info};
use tokio::sync::RwLock;

use bothan_core::worker::AssetWorker;

use crate::manager::crypto_asset_info::price::get_prices;
use crate::manager::crypto_asset_info::utils::find_diff;
use crate::proto::query::Price;
use crate::registry::Registry;

/// PriceServiceManager is used to manage price services.
///
/// ## Example
// ```no_run
// use std::collections::HashMap;
// use std::sync::Arc;
//
// use bothan_api::manager::PriceServiceManager;
// use bothan_core::service::Service;
// use bothan_coingecko::CoinGeckoServiceBuilder;
//
// #[tokio::main]
// async fn main() {
//     let registry = Arc::new(HashMap::new());
//     let mut manager = PriceServiceManager::new(registry, 60).unwrap();
//
//     let service = Box::new(CoinGeckoServiceBuilder::default().build().await.unwrap());
//
//     manager.add_service("mock".to_string(), service).await;
// }
// ```
pub struct CryptoAssetInfoManager {
    workers: RwLock<HashMap<String, Arc<dyn AssetWorker>>>,
    active_signal_ids: Arc<RwLock<HashSet<String>>>,
    registry: Arc<RwLock<Registry>>,
    stale_threshold: i64,
}

impl CryptoAssetInfoManager {
    /// Creates a new `PriceServiceManager` given a registry.
    pub fn new(registry: Arc<RwLock<Registry>>, stale_threshold: i64) -> Self {
        CryptoAssetInfoManager {
            workers: RwLock::new(HashMap::new()),
            active_signal_ids: Arc::new(RwLock::new(HashSet::new())),
            registry,
            stale_threshold,
        }
    }

    /// Add a service with an assigned name to the service map.
    pub async fn add_worker(&mut self, name: String, worker: Arc<dyn AssetWorker>) {
        self.workers.write().await.insert(name, worker);
    }

    pub async fn set_registry(&mut self, registry: Registry) {
        let mut writer = self.registry.write().await;
        *writer = registry;
    }

    pub async fn set_active_signal_ids(&mut self, signal_ids: Vec<String>) {
        let mut active_signal_ids = self.active_signal_ids.write().await;
        let new_set = signal_ids.into_iter().collect::<HashSet<String>>();

        let add = new_set
            .difference(&active_signal_ids)
            .collect::<Vec<&String>>();

        let rem = active_signal_ids
            .difference(&new_set)
            .collect::<Vec<&String>>();

        let registry = self.registry.read().await;
        let mut add_map = find_diff(add, &registry);
        let mut rem_map = find_diff(rem, &registry);

        let workers = self.workers.write().await;
        for (source, ids) in add_map.drain() {
            if let Some(worker) = workers.get(&source) {
                match worker.add_query_ids(ids).await {
                    Ok(_) => info!("Added query ids to {} worker", source),
                    Err(e) => error!("Worker {} failed to add query ids: {}", source, e),
                }
            }
        }

        for (source, ids) in rem_map.drain() {
            if let Some(worker) = workers.get(&source) {
                // TODO: remove
                let ids = ids.iter().map(|id| id.as_str()).collect::<Vec<&str>>();
                match worker.remove_query_ids(&ids).await {
                    Ok(_) => info!("Removed query ids from {} worker", source),
                    Err(e) => error!("Worker {} failed to remove query ids: {}", source, e),
                }
            }
        }

        *active_signal_ids = new_set;
    }

    /// Gets the [`PriceData`](crate::proto::query::query::PriceData) of the given signal ids.
    pub async fn get_prices(&mut self, ids: Vec<String>) -> anyhow::Result<Vec<Price>> {
        let registry = self.registry.read().await;
        let workers = self.workers.write().await;
        get_prices(ids, &registry, &workers, self.stale_threshold).await
    }
}
