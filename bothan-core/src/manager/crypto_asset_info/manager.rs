use std::collections::HashMap;
use std::sync::Arc;

use crate::ipfs::{error::Error as IpfsError, IpfsClient};
use crate::manager::crypto_asset_info::error::{
    PostHeartbeatError, PushMonitoringRecordError, SetRegistryError,
};
use crate::manager::crypto_asset_info::price::tasks::get_signal_price_states;
use crate::manager::crypto_asset_info::signal_ids::set_workers_query_ids;
use crate::manager::crypto_asset_info::types::{
    CryptoAssetManagerInfo, PriceSignalComputationRecord, PriceState, MONITORING_TTL,
};
use crate::manager::crypto_asset_info::worker::CryptoAssetWorker;
use crate::monitoring::{create_uuid, Client as MonitoringClient};
use bothan_lib::registry::{Invalid, Registry};
use bothan_lib::store::{RegistryStore, Store};
use mini_moka::sync::Cache;
use semver::{Version, VersionReq};
use serde_json::from_str;

pub struct CryptoAssetInfoManager<S: Store + 'static> {
    workers: HashMap<String, CryptoAssetWorker<S>>,
    store: RegistryStore<S>,
    stale_threshold: i64,
    ipfs_client: IpfsClient,
    bothan_version: Version,
    registry_version_requirement: VersionReq,
    monitoring_client: Option<Arc<MonitoringClient>>,
    monitoring_cache: Option<Cache<String, Arc<Vec<PriceSignalComputationRecord>>>>,
}

impl<S: Store + 'static> CryptoAssetInfoManager<S> {
    /// Creates a new `CryptoAssetInfoManager`.
    pub fn new(
        workers: HashMap<String, CryptoAssetWorker<S>>,
        store: RegistryStore<S>,
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

    /// Gets the `CryptoAssetManagerInfo`.
    pub async fn get_info(&self) -> Result<CryptoAssetManagerInfo, S::Error> {
        let bothan_version = self.bothan_version.to_string();
        let registry_hash = self
            .store
            .get_registry_ipfs_hash()
            .await?
            .unwrap_or(String::new()); // If value doesn't exist, return an empty string
        let registry_version_requirement = self.registry_version_requirement.to_string();
        let active_sources = self.workers.keys().cloned().collect();

        Ok(CryptoAssetManagerInfo::new(
            bothan_version,
            registry_hash,
            registry_version_requirement,
            active_sources,
            self.monitoring_client.is_some(),
        ))
    }

    /// Posts a heartbeat to the monitoring service.
    pub async fn post_heartbeat(&self) -> Result<String, PostHeartbeatError> {
        let client = self
            .monitoring_client
            .as_ref()
            .ok_or(PostHeartbeatError::MonitoringNotEnabled)?;

        let uuid = create_uuid();

        let active_sources = self.workers.keys().cloned().collect();
        let bothan_version = self.bothan_version.clone();
        let registry_hash = self
            .store
            .get_registry_ipfs_hash()
            .await
            .map_err(|_| PostHeartbeatError::FailedToGetRegistryHash)?
            .unwrap_or_else(|| "".to_string());

        client
            .post_heartbeat(uuid.clone(), active_sources, bothan_version, registry_hash)
            .await?
            .error_for_status()?;

        Ok(uuid)
    }

    /// Gets the `Price` of the given signal ids.
    pub async fn get_prices(
        &self,
        ids: Vec<String>,
    ) -> Result<(String, Vec<PriceState>), S::Error> {
        let registry = self.store.get_registry().await;

        let current_time = chrono::Utc::now().timestamp();
        let stale_cutoff = current_time - self.stale_threshold;

        let mut records = Vec::new();

        let price_states =
            get_signal_price_states(ids, &self.workers, &registry, stale_cutoff, &mut records)
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

    /// Pushes a monitoring record to the monitoring service.
    pub async fn push_monitoring_record(
        &self,
        uuid: String,
        tx_hash: String,
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
            .post_signal_record(uuid, tx_hash, records.clone())
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// Sets the registry from an IPFS hash.
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

        self.store
            .set_registry(registry, hash.to_string())
            .await
            .map_err(|_| SetRegistryError::FailedToSetRegistry)?;

        set_workers_query_ids(&self.workers, &self.store.get_registry().await).await;

        Ok(())
    }
}
