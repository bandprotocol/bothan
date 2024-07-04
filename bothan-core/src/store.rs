use std::collections::{HashMap, HashSet};

use tokio::sync::{Mutex, RwLock};

use crate::types::AssetInfo;

pub struct Store {
    asset_store: RwLock<HashMap<String, AssetInfo>>,
    query_ids: Mutex<HashSet<String>>,
}

impl Default for Store {
    fn default() -> Self {
        Store::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            asset_store: RwLock::new(HashMap::new()),
            query_ids: Mutex::new(HashSet::new()),
        }
    }

    pub async fn get_assets<K: AsRef<str>>(&self, ids: &[K]) -> Vec<crate::worker::AssetStatus> {
        let data_store = self.asset_store.read().await;
        let query_ids = self.query_ids.lock().await;

        ids.iter()
            .map(
                |id| match (query_ids.contains(id.as_ref()), data_store.get(id.as_ref())) {
                    (false, _) => crate::worker::AssetStatus::Unsupported,
                    (true, Some(asset)) => crate::worker::AssetStatus::Available(asset.clone()),
                    (true, None) => crate::worker::AssetStatus::Pending,
                },
            )
            .collect()
    }

    pub async fn get_all_assets(&self) -> Vec<AssetInfo> {
        self.asset_store.read().await.values().cloned().collect()
    }

    pub async fn set_asset<K: Into<String>>(&self, id: K, asset_info: AssetInfo) {
        let mut data_store = self.asset_store.write().await;
        data_store.insert(id.into(), asset_info);
    }

    pub async fn set_assets<K: Into<String>>(&self, assets: Vec<(K, AssetInfo)>) {
        let mut data_store = self.asset_store.write().await;
        for (id, asset) in assets {
            data_store.insert(id.into(), asset);
        }
    }

    pub async fn set_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<bool> {
        let mut query_ids = self.query_ids.lock().await;
        ids.into_iter()
            .map(|id| query_ids.insert(id.into()))
            .collect()
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