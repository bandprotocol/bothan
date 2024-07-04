use std::collections::{HashMap, HashSet};

use tokio::sync::{Mutex, RwLock};

use crate::types::AssetInfo;

#[derive(Debug)]
pub enum AssetStatus {
    Unsupported,
    Pending,
    Available(AssetInfo),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not started")]
    NotStarted,

    #[error("failed to modify query IDs: {0}")]
    ModifyQueryIDsFailed(String),
}

pub struct Storage {
    data_store: RwLock<HashMap<String, AssetInfo>>,
    query_ids: Mutex<HashSet<String>>,
}

impl Default for Storage {
    fn default() -> Self {
        Storage::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data_store: RwLock::new(HashMap::new()),
            query_ids: Mutex::new(HashSet::new()),
        }
    }

    pub async fn get_assets<K: AsRef<str>>(&self, ids: &[K]) -> Vec<AssetStatus> {
        let data_store = self.data_store.read().await;
        let query_ids = self.query_ids.lock().await;

        ids.iter()
            .map(|id| match (query_ids.contains(id.as_ref()), data_store.get(id.as_ref())) {
                (false, _) => AssetStatus::Unsupported,
                (true, Some(asset)) => AssetStatus::Available(asset.clone()),
                (true, None) => AssetStatus::Pending,
            })
            .collect()
    }

    pub async fn get_all_assets(&self) -> Vec<AssetInfo> {
        self.data_store.read().await.values().cloned().collect()
    }

    pub async fn set_asset<K: Into<String>>(&self, id: K, asset_info: AssetInfo) {
        let mut data_store = self.data_store.write().await;
        data_store.insert(id.into(), asset_info);
    }

    pub async fn set_assets<K: Into<String>>(&self, assets: Vec<(K, AssetInfo)>) {
        let mut data_store = self.data_store.write().await;
        for (id, asset) in assets {
            data_store.insert(id.into(), asset);
        }
    }

    pub async fn set_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<bool> {
        let mut query_ids = self.query_ids.lock().await;
        ids.into_iter().map(|id| query_ids.insert(id.into())).collect()
    }

    pub async fn remove_query_ids<K: AsRef<str>>(&self, ids: &[K]) -> Vec<bool> {
        let mut query_ids = self.query_ids.lock().await;
        ids.iter().map(|id| query_ids.remove(id.as_ref())).collect()
    }

    pub async fn get_query_ids(&self) -> Vec<String> {
        self.query_ids.lock().await.iter().cloned().collect()
    }

    pub async fn filter_existing_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<String> {
        let query_ids = self.query_ids.lock().await;
        ids.into_iter()
            .filter_map(|id| {
                let string_id = id.into();
                if !query_ids.contains(&string_id) {
                    Some(string_id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub async fn filter_missing_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<String> {
        let query_ids = self.query_ids.lock().await;
        ids.into_iter()
            .filter_map(|id| {
                let string_id = id.into();
                if query_ids.contains(&string_id) {
                    Some(string_id)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Type alias for a service result, which is either a valid result or an error.
/// The universal trait for all services that provide price data.
#[async_trait::async_trait]
pub trait AssetStore {
    async fn start(&mut self);
    async fn get_assets(&self, ids: &[&str]) -> Vec<AssetStatus>;
    async fn add_query_ids(&mut self, ids: &[&str]) -> Result<(), Error>;
    async fn remove_query_ids(&mut self, ids: &[&str]) -> Result<(), Error>;
    async fn get_query_ids(&self) -> Vec<String>;
}
