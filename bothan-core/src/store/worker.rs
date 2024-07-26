use std::collections::hash_map::Entry;
use std::collections::HashSet;
use std::ops::Deref;
use std::sync::Arc;

use tracing::error;

use crate::store::Store;
use crate::types::AssetInfo;
use crate::worker::AssetState;

pub struct WorkerStore {
    store: Arc<Store>,
    prefix: String,
}

impl WorkerStore {
    pub fn from_store<T: Into<String>>(store: Arc<Store>, prefix: T) -> Self {
        Self {
            store,
            prefix: prefix.into(),
        }
    }

    pub async fn get_assets<K: AsRef<str> + Send + Sync>(&self, ids: &[K]) -> Vec<AssetState> {
        let data_store = self.store.asset_store.read().await;
        let query_ids = self.store.query_ids.write().await;

        ids.iter()
            .map(|id| {
                let id = id.as_ref();
                let key = self.to_key(id);
                let in_query_set = query_ids
                    .get(&self.prefix)
                    .and_then(|set| set.get(id))
                    .is_some();

                match (in_query_set, data_store.get(&key)) {
                    (false, _) => AssetState::Unsupported,
                    (true, Some(asset)) => AssetState::Available(asset.clone()),
                    (true, None) => AssetState::Pending,
                }
            })
            .collect()
    }

    pub async fn set_asset<K: AsRef<str>>(&self, id: K, asset_info: AssetInfo) {
        let mut data_store = self.store.asset_store.write().await;
        data_store.insert(self.to_key(id.as_ref()), asset_info);

        if let Err(e) = self.store.save_state("asset_store", data_store.deref()) {
            error!("failed to save asset_store state: {}", e);
        }
    }

    pub async fn set_assets<K: AsRef<str>>(&self, assets: Vec<(K, AssetInfo)>) {
        let mut data_store = self.store.asset_store.write().await;
        for (id, asset) in assets {
            data_store.insert(self.to_key(id.as_ref()), asset);
        }
    }

    pub async fn add_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<String> {
        let mut all_query_ids = self.store.query_ids.write().await;
        let ids = ids.into_iter().map(Into::into).collect::<Vec<String>>();

        let res = match all_query_ids.entry(self.prefix.clone()) {
            Entry::Occupied(mut o) => {
                let set = o.get_mut();
                ids.into_iter()
                    .filter_map(|id| {
                        let key = self.to_key(&id);
                        set.insert(key).then_some(id)
                    })
                    .collect()
            }
            Entry::Vacant(v) => {
                v.insert(HashSet::from_iter(ids.iter().cloned()));
                ids
            }
        };

        if let Err(e) = self.store.save_state("query_ids", all_query_ids.deref()) {
            error!("failed to save query_ids state: {}", e);
        }

        res
    }

    pub async fn remove_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<String> {
        let mut all_query_ids = self.store.query_ids.write().await;
        let ids = ids.into_iter().map(Into::into).collect::<Vec<String>>();

        let res = match all_query_ids.entry(self.prefix.clone()) {
            Entry::Occupied(mut o) => {
                let set = o.get_mut();
                ids.into_iter()
                    .filter_map(|id| {
                        let key = self.to_key(&id);
                        set.remove(&key).then_some(id)
                    })
                    .collect()
            }
            Entry::Vacant(_) => vec![],
        };

        if let Err(e) = self.store.save_state("query_ids", all_query_ids.deref()) {
            error!("failed to save query_ids state: {}", e);
        }

        res
    }

    pub async fn get_query_ids(&self) -> HashSet<String> {
        match self.store.query_ids.read().await.get(&self.prefix) {
            Some(ids) => ids.clone(),
            None => HashSet::new(),
        }
    }

    fn to_key(&self, id: &str) -> String {
        format!("{}+{}", self.prefix, id)
    }
}
