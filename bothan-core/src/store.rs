use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use rust_rocksdb::{Options, DB};
use tokio::sync::RwLock;

pub use manager::ManagerStore;
pub use worker::WorkerStore;

use crate::registry::Registry;
use crate::store::errors::Error;
use crate::store::types::Key;
use crate::types::AssetInfo;

pub mod errors;
mod manager;
mod types;
mod worker;

#[derive(Clone)]
pub struct SharedStore {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    asset_store: HashMap<String, AssetInfo>,
    query_ids: HashMap<String, HashSet<String>>,
    active_signal_ids: HashSet<String>,
    registry: Registry,
    db: DB,
}

impl SharedStore {
    /// Create a new shared store with the given registry and flush path. If the store already exists at the
    /// given path, it will be restored and the registry will be overwritten.
    pub async fn new(registry: Registry, flush_path: &Path) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let inner = Inner {
            asset_store: HashMap::new(),
            query_ids: HashMap::new(),
            active_signal_ids: HashSet::new(),
            registry,
            db: DB::open(&opts, flush_path)?,
        };

        let store = Self {
            inner: Arc::new(RwLock::new(inner)),
        };

        Ok(store)
    }

    pub async fn restore(&mut self) -> Result<(), Error> {
        let mut inner = self.inner.write().await;

        let asset_store = inner
            .db
            .get(Key::AssetStore.as_bytes())?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        let query_ids = inner
            .db
            .get(Key::QueryIds.as_bytes())?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        let active_signal_ids = inner
            .db
            .get(Key::ActiveSignalIDs.as_bytes())?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        let registry = inner
            .db
            .get(Key::Registry.as_bytes())?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;

        if let Some(query_ids) = query_ids {
            inner.query_ids = query_ids;
        }
        if let Some(asset_store) = asset_store {
            inner.asset_store = asset_store;
        }
        if let Some(active_signal_ids) = active_signal_ids {
            inner.active_signal_ids = active_signal_ids;
        }
        if let Some(registry) = registry {
            inner.registry = registry;
        }

        Ok(())
    }

    pub fn create_manager_store(this: &Self) -> ManagerStore {
        ManagerStore::new(this.clone())
    }

    pub fn create_worker_store<T: Into<String>>(this: &Self, prefix: T) -> WorkerStore {
        WorkerStore::new(this.clone(), prefix.into())
    }

    async fn save_state(&self, key: &Key) -> Result<(), Error> {
        let inner = self.inner.write().await;

        let serialized = match key {
            Key::AssetStore => bincode::serialize(&inner.asset_store),
            Key::QueryIds => bincode::serialize(&inner.query_ids),
            Key::ActiveSignalIDs => bincode::serialize(&inner.active_signal_ids),
            Key::Registry => bincode::serialize(&inner.registry),
        }?;

        inner.db.put(key.as_bytes(), serialized)?;
        inner.db.flush()?;

        Ok(())
    }

    async fn get_active_signal_ids(&self) -> HashSet<String> {
        self.inner.read().await.active_signal_ids.clone()
    }

    async fn set_active_signal_ids(&self, signal_ids: HashSet<String>) {
        let mut inner = self.inner.write().await;
        inner.active_signal_ids = signal_ids;
    }

    async fn get_registry(&self) -> Registry {
        self.inner.read().await.registry.clone()
    }

    async fn set_registry(&self, registry: Registry) {
        let mut inner = self.inner.write().await;
        inner.registry = registry;
    }

    async fn get_query_ids<P: AsRef<str>>(&self, prefix: &P) -> HashSet<String> {
        self.inner
            .read()
            .await
            .query_ids
            .get(prefix.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    async fn contains_query_id<P, K>(&self, prefix: &P, id: &K) -> bool
    where
        P: AsRef<str>,
        K: AsRef<str>,
    {
        self.inner
            .read()
            .await
            .query_ids
            .get(prefix.as_ref())
            .map_or(false, |s| s.contains(id.as_ref()))
    }

    async fn insert_query_ids<P, K>(&self, prefix: P, ids: Vec<K>) -> Vec<bool>
    where
        P: Into<String>,
        K: Into<String>,
    {
        let mut inner = self.inner.write().await;
        let inner_query_id_map = inner
            .query_ids
            .entry(prefix.into())
            .or_insert_with(HashSet::new);

        ids.into_iter()
            .map(|id| inner_query_id_map.insert(id.into()))
            .collect()
    }

    async fn remove_query_ids<P, K>(&self, prefix: &P, ids: &[K]) -> Vec<bool>
    where
        P: AsRef<str>,
        K: AsRef<str>,
    {
        let mut inner = self.inner.write().await;
        if let Some(inner_query_id_map) = inner.query_ids.get_mut(prefix.as_ref()) {
            ids.iter()
                .map(|id| inner_query_id_map.remove(id.as_ref()))
                .collect()
        } else {
            vec![false; ids.len()]
        }
    }

    async fn get_asset_info<P, K>(&self, prefix: &P, key: K) -> Option<AssetInfo>
    where
        P: AsRef<str>,
        K: AsRef<str>,
    {
        let key = format!("{}+{}", prefix.as_ref(), key.as_ref());
        self.inner.read().await.asset_store.get(&key).cloned()
    }

    async fn insert_asset_info<P, K>(&self, prefix: &P, key: K, value: AssetInfo)
    where
        P: AsRef<str>,
        K: AsRef<str>,
    {
        let mut inner = self.inner.write().await;
        let key = format!("{}+{}", prefix.as_ref(), key.as_ref());
        inner.asset_store.insert(key, value);
    }

    async fn insert_asset_infos<P, K>(&self, prefix: &P, assets: Vec<(K, AssetInfo)>)
    where
        P: AsRef<str>,
        K: AsRef<str>,
    {
        let mut inner = self.inner.write().await;
        for (id, asset) in assets {
            let key = format!("{}+{}", prefix.as_ref(), id.as_ref());
            inner.asset_store.insert(key, asset);
        }
    }
}

impl PartialEq for SharedStore {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}
