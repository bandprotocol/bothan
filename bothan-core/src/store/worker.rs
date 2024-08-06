use std::collections::HashSet;

use futures::future::join_all;
use tracing::{error, info};

use crate::store::types::Key;
use crate::store::SharedStore;
use crate::types::AssetInfo;
use crate::worker::AssetState;

pub struct WorkerStore {
    store: SharedStore,
    prefix: String,
}

impl WorkerStore {
    pub fn new<T: Into<String>>(store: SharedStore, prefix: T) -> Self {
        Self {
            store,
            prefix: prefix.into(),
        }
    }

    pub async fn get_assets<K: AsRef<str>>(&self, ids: &[K]) -> Vec<AssetState> {
        let asset_states = ids.iter().map(|id| async move {
            if !self.store.contains_query_id(&self.prefix, id).await {
                return AssetState::Unsupported;
            }

            match self.store.get_asset_info(&self.prefix, id).await {
                Some(asset) => AssetState::Available(asset),
                None => AssetState::Pending,
            }
        });
        join_all(asset_states).await
    }

    pub async fn set_asset<K: AsRef<str>>(&self, id: K, asset_info: AssetInfo) {
        self.store
            .insert_asset_info(&self.prefix, id, asset_info)
            .await;

        let key = Key::AssetStore;
        match self.store.save_state(&key).await {
            Ok(_) => info!("saved {} state successfully", key),
            Err(e) => error!("failed to save {} state: {}", key, e),
        }
    }

    pub async fn set_assets<K: AsRef<str>>(&self, assets: Vec<(K, AssetInfo)>) {
        self.store.insert_asset_infos(&self.prefix, assets).await;

        let key = Key::AssetStore;
        match self.store.save_state(&key).await {
            Ok(_) => info!("saved {} state successfully", key),
            Err(e) => error!("failed to save {} state: {}", key, e),
        }
    }

    pub async fn add_query_ids<K>(&self, ids: Vec<K>) -> Vec<K>
    where
        K: Into<String> + Clone,
    {
        let changes = self.store.insert_query_ids(&self.prefix, ids.clone()).await;

        let key = Key::QueryIds;
        match self.store.save_state(&key).await {
            Ok(_) => info!("saved {} state successfully", key),
            Err(e) => error!("failed to save {} state: {}", key, e),
        }

        ids.into_iter()
            .zip(changes.into_iter())
            .filter(|&(_, changed)| changed)
            .map(|(id, _)| id)
            .collect()
    }

    pub async fn remove_query_ids<K>(&self, ids: Vec<K>) -> Vec<K>
    where
        K: Into<String> + AsRef<str>,
    {
        let changes = self.store.remove_query_ids(&self.prefix, &ids).await;

        let key = Key::QueryIds;
        match self.store.save_state(&key).await {
            Ok(_) => info!("saved {} state successfully", key),
            Err(e) => error!("failed to save {} state: {}", key, e),
        }

        ids.into_iter()
            .zip(changes.into_iter())
            .filter(|&(_, changed)| changed)
            .map(|(id, _)| id)
            .collect()
    }

    pub async fn get_query_ids(&self) -> HashSet<String> {
        self.store.get_query_ids(&self.prefix).await
    }
}
