use std::collections::hash_map::Entry;
use std::collections::HashSet;

use tracing::{error, info};

use crate::store::types::{ASSET_STORE_KEY, QUERY_IDS_KEY};
use crate::store::Store;
use crate::types::AssetInfo;
use crate::worker::AssetState;

pub struct WorkerStore {
    store: Store,
    prefix: String,
}

impl WorkerStore {
    pub fn new<T: Into<String>>(store: Store, prefix: T) -> Self {
        Self {
            store,
            prefix: prefix.into(),
        }
    }

    pub async fn get_assets<K: AsRef<str> + Send + Sync>(&self, ids: &[K]) -> Vec<AssetState> {
        let store = self.store.inner.lock().await;

        ids.iter()
            .map(|id| {
                let id = id.as_ref();
                let key = self.to_key(id);
                let in_query_set = store
                    .query_ids
                    .get(&self.prefix)
                    .and_then(|set| set.get(id))
                    .is_some();

                match (in_query_set, store.asset_store.get(&key)) {
                    (false, _) => AssetState::Unsupported,
                    (true, Some(asset)) => AssetState::Available(asset.clone()),
                    (true, None) => AssetState::Pending,
                }
            })
            .collect()
    }

    pub async fn set_asset<K: AsRef<str>>(&self, id: K, asset_info: AssetInfo) {
        let mut store = self.store.inner.lock().await;
        store
            .asset_store
            .insert(self.to_key(id.as_ref()), asset_info);

        let save_result = self
            .store
            .save_state(ASSET_STORE_KEY, &store.asset_store)
            .await;

        match save_result {
            Ok(_) => info!("asset_store state saved successfully"),
            Err(e) => error!("failed to save asset_store state: {}", e),
        }
    }

    pub async fn set_assets<K: AsRef<str>>(&self, assets: Vec<(K, AssetInfo)>) {
        let mut store = self.store.inner.lock().await;
        for (id, asset) in assets {
            store.asset_store.insert(self.to_key(id.as_ref()), asset);
        }

        let save_result = self
            .store
            .save_state(ASSET_STORE_KEY, &store.asset_store)
            .await;

        match save_result {
            Ok(_) => info!("asset_store state saved successfully"),
            Err(e) => error!("failed to save asset_store state: {}", e),
        }
    }

    pub async fn add_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<String> {
        let mut store = self.store.inner.lock().await;
        let ids = ids.into_iter().map(Into::into).collect::<Vec<String>>();

        let res = match store.query_ids.entry(self.prefix.clone()) {
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

        let save_result = self.store.save_state(QUERY_IDS_KEY, &store.query_ids).await;

        match save_result {
            Ok(_) => info!("query_ids state saved successfully"),
            Err(e) => error!("failed to save query_ids state: {}", e),
        }

        res
    }

    pub async fn remove_query_ids<K: Into<String>>(&self, ids: Vec<K>) -> Vec<String> {
        let mut store = self.store.inner.lock().await;
        let ids = ids.into_iter().map(Into::into).collect::<Vec<String>>();

        let res = match store.query_ids.entry(self.prefix.clone()) {
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

        let save_result = self.store.save_state(QUERY_IDS_KEY, &store.query_ids).await;

        match save_result {
            Ok(_) => info!("query_ids state saved successfully"),
            Err(e) => error!("failed to save query_ids state: {}", e),
        }

        res
    }

    pub async fn get_query_ids(&self) -> HashSet<String> {
        match self.store.inner.lock().await.query_ids.get(&self.prefix) {
            Some(ids) => ids.clone(),
            None => HashSet::new(),
        }
    }

    fn to_key(&self, id: &str) -> String {
        format!("{}+{}", self.prefix, id)
    }
}
