use std::collections::HashMap;
use std::sync::Arc;

use mini_moka::sync::Cache;
use semver::{Version, VersionReq};
use serde_json::from_str;

use crate::ipfs::{error::Error as IpfsError, IpfsClient};
use crate::manager::crypto_asset_info::error::{
    PostHeartbeatError, PushMonitoringRecordError, SetRegistryError,
};
use crate::manager::crypto_asset_info::price::tasks::get_signal_price_states;
use crate::manager::crypto_asset_info::signal_ids::set_workers_query_ids;
use crate::manager::crypto_asset_info::types::{
    CryptoAssetManagerInfo, PriceSignalComputationRecords, PriceState, MONITORING_TTL,
};
use crate::monitoring::records::SignalComputationRecords;
use crate::monitoring::{create_uuid, Client as MonitoringClient};
use crate::registry::{Invalid, Registry};
use crate::store::error::Error as StoreError;
use crate::store::ManagerStore;
use crate::worker::AssetWorker;

pub struct CryptoAssetInfoManager<'a> {
    workers: HashMap<String, Arc<dyn AssetWorker + 'a>>,
    store: ManagerStore,
    stale_threshold: i64,
    ipfs_client: IpfsClient,
    bothan_version: Version,
    registry_version_requirement: VersionReq,
    monitoring_client: Option<Arc<MonitoringClient>>,
    monitoring_cache: Option<Cache<String, Arc<PriceSignalComputationRecords>>>,
}

impl<'a> CryptoAssetInfoManager<'a> {
    pub fn new(
        workers: HashMap<String, Arc<dyn AssetWorker + 'a>>,
        store: ManagerStore,
        ipfs_client: IpfsClient,
        stale_threshold: i64,
        bothan_version: Version,
        registry_version_requirement: VersionReq,
        monitoring_client: Option<Arc<MonitoringClient>>,
    ) -> Self {
        let monitoring_cache = monitoring_client
            .as_ref()
            .map(|_| Cache::builder().time_to_idle(MONITORING_TTL).build());

        CryptoAssetInfoManager {
            workers,
            store,
            stale_threshold,
            ipfs_client,
            bothan_version,
            registry_version_requirement,
            monitoring_client,
            monitoring_cache,
        }
    }

    pub async fn get_info(&self) -> Result<CryptoAssetManagerInfo, StoreError> {
        let bothan_version = self.bothan_version.to_string();
        let registry_hash = self
            .store
            .get_registry_hash()
            .await?
            .unwrap_or(String::new()); // If value doesn't exist, return an empty string
        let registry_version_requirement = self.registry_version_requirement.to_string();

        Ok(CryptoAssetManagerInfo::new(
            bothan_version,
            registry_hash,
            registry_version_requirement,
        ))
    }

    pub async fn post_heartbeat(&self) -> Result<String, PostHeartbeatError> {
        let client = self
            .monitoring_client
            .as_ref()
            .ok_or(PostHeartbeatError::MonitoringNotEnabled)?;

        let uuid = create_uuid();

        let supported_sources = self.current_worker_set().await;
        let bothan_version = self.bothan_version.clone();
        let registry_hash = self
            .store
            .get_registry_hash()
            .await
            .map_err(|_| PostHeartbeatError::FailedToGetRegistryHash)?
            .unwrap_or_else(|| "".to_string());

        client
            .post_heartbeat(
                uuid.clone(),
                supported_sources,
                bothan_version,
                registry_hash,
            )
            .await?
            .error_for_status()?;

        Ok(uuid)
    }

    pub async fn current_worker_set(&self) -> Vec<String> {
        self.workers.keys().cloned().collect()
    }

    /// Gets the `Price` of the given signal ids.
    pub async fn get_prices(
        &self,
        ids: Vec<String>,
    ) -> Result<(String, Vec<PriceState>), StoreError> {
        let registry = self.store.get_registry().await;

        let current_time = chrono::Utc::now().timestamp();
        let stale_cutoff = current_time - self.stale_threshold;
        let active_signals = self.store.get_active_signal_ids().await?;

        let mut records = SignalComputationRecords::default();

        let price_states = get_signal_price_states(
            ids,
            &self.workers,
            &registry,
            &active_signals,
            stale_cutoff,
            &mut records,
        )
        .await;

        let uuid = create_uuid();
        if let Some(cache) = &self.monitoring_cache {
            // Note: We wrap this in arc since records is quite large,
            // and we don't want to clone the entire value.
            // As the reference will only be used to push to monitoring, we can
            // assume that all references will be dropped after the tti expires
            cache.insert(uuid.clone(), Arc::new(records));
        }

        Ok((uuid, price_states))
    }

    // TODO: implement tx hash mapping into monitoring
    pub async fn push_monitoring_record(
        &self,
        uuid: String,
        _tx_hash: String,
    ) -> Result<(), PushMonitoringRecordError> {
        let client = self
            .monitoring_client
            .as_ref()
            .ok_or(PushMonitoringRecordError::MonitoringNotEnabled)?;
        let cache = self
            .monitoring_cache
            .as_ref()
            .ok_or(PushMonitoringRecordError::MonitoringNotEnabled)?;

        let records = cache
            .get(&uuid)
            .ok_or(PushMonitoringRecordError::RecordNotFound)?;
        client
            .post_arced_signal_record(uuid, records.clone())
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn set_registry_from_ipfs(
        &self,
        hash: &str,
        version: Version,
    ) -> Result<(), SetRegistryError> {
        if !self.registry_version_requirement.matches(&version) {
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

        self.store.set_registry(registry, hash.to_string()).await?;

        set_workers_query_ids(&self.workers, &self.store.get_registry().await).await;

        Ok(())
    }
}
