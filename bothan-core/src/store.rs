use std::collections::{HashMap, HashSet};
use std::path::Path;

pub use manager::ManagerStore;
use rust_rocksdb::{Options, DB};
use serde::Serialize;
use tokio::sync::RwLock;
use tracing::info;
pub use worker::WorkerStore;

use crate::registry::Registry;
use crate::store::errors::Error;
use crate::types::AssetInfo;

pub mod errors;
mod manager;
mod worker;

pub struct Store {
    asset_store: RwLock<HashMap<String, AssetInfo>>,
    query_ids: RwLock<HashMap<String, HashSet<String>>>,
    active_signal_ids: RwLock<HashSet<String>>,
    registry: RwLock<Registry>,
    db: DB,
}

impl Store {
    /// Create a new store with the given registry and flush path. If the store already exists at the
    /// given path, it will be restored and the registry will be overwritten.
    pub async fn new(registry: Registry, flush_path: &Path) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, flush_path)?;

        let mut store = Self {
            asset_store: RwLock::new(HashMap::new()),
            query_ids: RwLock::new(HashMap::new()),
            active_signal_ids: RwLock::new(HashSet::new()),
            registry: RwLock::new(registry),
            db,
        };

        store.restore();

        Ok(store)
    }

    fn restore(&mut self) {
        if let Ok(Some(bytes)) = self.db.get("asset_store") {
            if let Ok(Some(asset_store)) = bincode::deserialize(bytes.as_slice()) {
                self.asset_store = RwLock::new(asset_store);
            }
        }

        if let Ok(Some(bytes)) = self.db.get("query_ids") {
            if let Ok(Some(query_ids)) = bincode::deserialize(bytes.as_slice()) {
                self.query_ids = RwLock::new(query_ids);
            }
        }

        if let Ok(Some(bytes)) = self.db.get("active_signal_ids") {
            if let Ok(Some(active_signal_ids)) = bincode::deserialize(bytes.as_slice()) {
                self.active_signal_ids = RwLock::new(active_signal_ids);
            }
        }
    }

    fn save_state<K, V>(&self, key: K, val: &V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: Serialize + ?Sized,
    {
        info!("saving state");
        let serialized = bincode::serialize(&val)?;
        self.db.put(key, serialized)?;
        self.db.flush()?;
        info!("saved state");
        Ok(())
    }
}
