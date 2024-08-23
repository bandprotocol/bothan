use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use semver::{Version, VersionReq};
use serde_json::from_str;
use tokio::sync::RwLock;

use crate::ipfs::{errors::Error as IpfsError, IpfsClient};
use crate::manager::crypto_asset_info::error::{SetActiveSignalError, SetRegistryError};
use crate::manager::crypto_asset_info::price::tasks::get_signal_price_states;
use crate::manager::crypto_asset_info::signal_ids::set_workers_query_ids;
use crate::manager::crypto_asset_info::types::PriceState;
use crate::registry::{Invalid, Registry};
use crate::store::errors::Error as StoreError;
use crate::store::ManagerStore;
use crate::worker::AssetWorker;

pub struct CryptoAssetInfoManager<'a> {
    workers: RwLock<HashMap<String, Arc<dyn AssetWorker + 'a>>>,
    store: ManagerStore,
    stale_threshold: i64,
    ipfs_client: IpfsClient,
    version_req: VersionReq,
}

impl<'a> CryptoAssetInfoManager<'a> {
    pub fn new(
        store: ManagerStore,
        ipfs_client: IpfsClient,
        stale_threshold: i64,
        version_req: VersionReq,
    ) -> Self {
        CryptoAssetInfoManager {
            workers: RwLock::new(HashMap::new()),
            store,
            stale_threshold,
            ipfs_client,
            version_req,
        }
    }

    /// Adds a worker with an assigned name.
    pub async fn add_worker(&mut self, name: String, worker: Arc<dyn AssetWorker + 'a>) {
        self.workers.write().await.insert(name, worker);
    }

    /// Sets the active signal ids of the manager.
    pub async fn set_active_signal_ids(
        &mut self,
        signal_ids: Vec<String>,
    ) -> Result<(), SetActiveSignalError> {
        let new_active_signal_ids = signal_ids.iter().cloned().collect::<HashSet<String>>();

        let workers = self.workers.write().await;
        let registry = self.store.get_registry().await;

        set_workers_query_ids(&workers, &new_active_signal_ids, &registry).await?;
        self.store.set_active_signal_ids(signal_ids).await?;

        Ok(())
    }

    /// Gets the `Price` of the given signal ids.
    pub async fn get_prices(&self, ids: Vec<String>) -> Result<Vec<PriceState>, StoreError> {
        let registry = self.store.get_registry().await;
        let workers = self.workers.read().await;

        let current_time = chrono::Utc::now().timestamp();
        let stale_cutoff = current_time - self.stale_threshold;
        let active_signals = self.store.get_active_signal_ids().await?;

        Ok(get_signal_price_states(ids, &workers, &registry, &active_signals, stale_cutoff).await)
    }

    pub async fn set_registry_from_ipfs(
        &mut self,
        hash: &str,
        version: Version,
    ) -> Result<(), SetRegistryError> {
        if !self.version_req.matches(&version) {
            return Err(SetRegistryError::UnsupportedVersion);
        };

        let text = self
            .ipfs_client
            .get_ipfs(&hash)
            .await
            .map_err(|e| match e {
                IpfsError::DoesNotExist => SetRegistryError::InvalidHash,
                IpfsError::NonZeroStatus(code) => SetRegistryError::FailedToRetrieve(format!(
                    "failed to get registry with non-zero status code: {code}"
                )),
                IpfsError::RequestFailed(e) => SetRegistryError::FailedToRetrieve(format!(
                    "failed to get registry with error: {e}"
                )),
            })?;

        let unchecked_registry =
            from_str::<Registry<Invalid>>(&text).map_err(|_| SetRegistryError::FailedToParse)?;
        let registry = unchecked_registry
            .validate()
            .map_err(|e| SetRegistryError::InvalidRegistry(e.to_string()))?;

        self.store.set_registry(registry).await?;
        Ok(())
    }
}
