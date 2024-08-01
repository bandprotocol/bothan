use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use rust_rocksdb::{Options, DB};
use serde::Serialize;
use tokio::sync::Mutex;

pub use manager::ManagerStore;
pub use worker::WorkerStore;

use crate::registry::Registry;
use crate::store::errors::Error;
use crate::types::AssetInfo;

pub mod errors;
mod manager;
mod worker;

#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<StoreInner>>,
    db: Arc<DB>,
}

struct StoreInner {
    asset_store: HashMap<String, AssetInfo>,
    query_ids: HashMap<String, HashSet<String>>,
    active_signal_ids: HashSet<String>,
    registry: Registry,
}

impl Store {
    /// Create a new store with the given registry and flush path. If the store already exists at the
    /// given path, it will be restored and the registry will be overwritten.
    pub async fn new(registry: Registry, flush_path: &Path) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = Arc::new(DB::open(&opts, flush_path)?);

        let inner = StoreInner {
            asset_store: HashMap::new(),
            query_ids: HashMap::new(),
            active_signal_ids: HashSet::new(),
            registry,
        };
        let store = Self {
            inner: Arc::new(Mutex::new(inner)),
            db,
        };

        Ok(store)
    }

    pub async fn restore(&mut self) -> Result<(), Error> {
        let asset_store = self
            .db
            .get("asset_store")?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        let query_ids = self
            .db
            .get("query_ids")?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        let active_signal_ids = self
            .db
            .get("active_signal_ids")?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        let registry = self
            .db
            .get("registry")?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;

        let mut inner = self.inner.lock().await;
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

    async fn save_state<K, V>(&self, key: K, val: &V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: Serialize + ?Sized,
    {
        let serialized = bincode::serialize(&val)?;

        self.db.put(key, serialized)?;
        self.db.flush()?;

        Ok(())
    }

    pub fn create_manager_store(this: &Self) -> ManagerStore {
        ManagerStore::new(this.clone())
    }

    pub fn create_worker_store<T: Into<String>>(this: &Self, prefix: T) -> WorkerStore {
        WorkerStore::new(this.clone(), prefix.into())
    }
}

impl PartialEq for Store {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner) && Arc::ptr_eq(&self.db, &other.db)
    }
}
