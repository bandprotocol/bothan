use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use rust_rocksdb::{Options, DB};
use tokio::sync::RwLock;
use tracing::debug;

pub use manager::ManagerStore;
pub use worker::WorkerStore;

use crate::registry::{Registry, Valid};
use crate::store::error::Error;
use crate::store::types::Key;
use crate::types::AssetInfo;

pub mod error;
mod manager;
mod types;
mod worker;

#[derive(Clone)]
pub struct SharedStore {
    inner: Arc<RwLock<Inner>>,
}

pub type AssetStore = HashMap<String, AssetInfo>;
pub type QueryIDs = HashSet<String>;
pub type ActiveSignalIDs = HashSet<String>;

struct Inner {
    registry: Registry<Valid>,
    db: DB,
}

impl SharedStore {
    /// Create a new shared store with the given registry and flush path.
    pub async fn new(registry: Registry<Valid>, flush_path: &Path) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let inner = Inner {
            registry,
            db: DB::open(&opts, flush_path)?,
        };

        let store = Self {
            inner: Arc::new(RwLock::new(inner)),
        };

        Ok(store)
    }

    /// Restore the store's registry from the database
    pub async fn restore(&mut self) -> Result<(), Error> {
        let mut inner = self.inner.write().await;

        let registry = inner
            .db
            .get(Key::Registry.to_prefixed_bytes())?
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;

        if let Some(registry) = registry {
            debug!("loaded registry");
            inner.registry = registry;
        }

        Ok(())
    }

    pub fn create_manager_store(&self) -> ManagerStore {
        ManagerStore::new(self.clone())
    }

    pub fn create_worker_store<T: Into<String>>(&self, prefix: T) -> WorkerStore {
        WorkerStore::new(self.clone(), prefix.into())
    }

    async fn get_active_signal_ids(&self) -> Result<Option<ActiveSignalIDs>, Error> {
        let serialized = self
            .inner
            .read()
            .await
            .db
            .get(Key::ActiveSignalIDs.to_prefixed_bytes())?;
        let active_signal_ids = serialized
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        Ok(active_signal_ids)
    }

    async fn set_active_signal_ids(&self, signal_ids: HashSet<String>) -> Result<(), Error> {
        let serialized = bincode::serialize(&signal_ids)?;
        self.inner
            .write()
            .await
            .db
            .put(Key::ActiveSignalIDs.to_prefixed_bytes(), serialized)?;
        Ok(())
    }

    async fn get_registry(&self) -> Registry<Valid> {
        self.inner.read().await.registry.clone()
    }

    async fn set_registry(&self, registry: Registry<Valid>) -> Result<(), Error> {
        let mut inner = self.inner.write().await;
        let serialized = bincode::serialize(&registry)?;
        inner
            .db
            .put(Key::Registry.to_prefixed_bytes(), serialized)?;
        inner.registry = registry;
        Ok(())
    }

    async fn get_query_ids<S: AsRef<str>>(&self, source_id: &S) -> Result<Option<QueryIDs>, Error> {
        let key = Key::QueryIDs {
            source_id: source_id.as_ref(),
        };

        let serialized = self.inner.read().await.db.get(key.to_prefixed_bytes())?;
        let query_ids = serialized
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        Ok(query_ids)
    }

    async fn contains_query_id<S, I>(&self, source_id: &S, id: &I) -> Result<bool, Error>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        match self.get_query_ids(source_id).await {
            Ok(Some(query_ids)) => Ok(query_ids.contains(id.as_ref())),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn set_query_ids<S: AsRef<str>>(
        &self,
        source_id: &S,
        query_ids: QueryIDs,
    ) -> Result<(), Error> {
        let key = Key::QueryIDs {
            source_id: source_id.as_ref(),
        };

        let serialized = bincode::serialize(&query_ids)?;
        self.inner
            .write()
            .await
            .db
            .put(key.to_prefixed_bytes(), serialized)?;
        Ok(())
    }

    async fn insert_query_ids<S, I>(&self, source_id: &S, ids: Vec<I>) -> Result<Vec<bool>, Error>
    where
        S: AsRef<str>,
        I: Into<String>,
    {
        let mut query_ids = self.get_query_ids(&source_id).await?.unwrap_or_default();
        let inserted = ids
            .into_iter()
            .map(|id| query_ids.insert(id.into()))
            .collect();

        self.set_query_ids(&source_id, query_ids).await?;
        Ok(inserted)
    }

    async fn remove_query_ids<S, I>(&self, source_id: &S, ids: &[I]) -> Result<Vec<bool>, Error>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let mut current_set = self.get_query_ids(&source_id).await?.unwrap_or_default();
        let removed = ids
            .iter()
            .map(|id| current_set.remove(id.as_ref()))
            .collect();

        self.set_query_ids(&source_id, current_set).await?;
        Ok(removed)
    }

    async fn get_asset_info<S, I>(&self, source_id: &S, id: &I) -> Result<Option<AssetInfo>, Error>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let key = Key::AssetStore {
            source_id: source_id.as_ref(),
            id: id.as_ref(),
        };

        let serialized = self.inner.read().await.db.get(key.to_prefixed_bytes())?;
        let asset_info = serialized
            .map(|b| bincode::deserialize(b.as_slice()))
            .transpose()?;
        Ok(asset_info)
    }

    async fn insert_asset_info<S, I>(
        &self,
        source_id: &S,
        id: &I,
        asset_info: AssetInfo,
    ) -> Result<(), Error>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let key = Key::AssetStore {
            source_id: source_id.as_ref(),
            id: id.as_ref(),
        };

        let serialized = bincode::serialize(&asset_info)?;
        self.inner
            .write()
            .await
            .db
            .put(key.to_prefixed_bytes(), serialized)?;
        Ok(())
    }

    async fn insert_asset_infos<S, I>(
        &self,
        source_id: &S,
        assets: Vec<(I, AssetInfo)>,
    ) -> Result<(), Error>
    where
        S: AsRef<str>,
        I: AsRef<str>,
    {
        let inner = self.inner.write().await;
        for (id, asset_info) in assets {
            let key = Key::AssetStore {
                source_id: source_id.as_ref(),
                id: id.as_ref(),
            };
            let serialized = bincode::serialize(&asset_info)?;
            inner.db.put(key.to_prefixed_bytes(), serialized)?;
        }
        Ok(())
    }
}

impl PartialEq for SharedStore {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}
