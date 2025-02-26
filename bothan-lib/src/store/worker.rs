use std::collections::HashSet;
use std::hash::RandomState;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::store::Store;
use crate::types::{AssetInfo, AssetState};

#[derive(Clone)]
pub struct WorkerStore<S: Store> {
    store: S,
    prefix: String,
    query_ids_lock: Arc<Mutex<()>>,
}

impl<S: Store> WorkerStore<S> {
    /// Creates a new WorkerStore with the specified store and unique prefix key.
    pub fn new<T: Into<String>>(store: &S, prefix: T) -> Self {
        Self {
            store: store.clone(),
            prefix: prefix.into(),
            query_ids_lock: Arc::new(Mutex::new(())),
        }
    }

    /// Get the asset state for the specified query id.
    pub async fn get_asset(&self, id: &str) -> Result<AssetState, S::Error> {
        if !self.store.contains_query_id(&self.prefix, id).await? {
            return Ok(AssetState::Unsupported);
        }

        match self.store.get_asset_info(&self.prefix, id).await? {
            Some(asset) => Ok(AssetState::Available(asset)),
            None => Ok(AssetState::Pending),
        }
    }

    /// Set the asset state for the specified query id.
    pub async fn set_asset_info(&self, asset: AssetInfo) -> Result<(), S::Error> {
        self.store.insert_asset_info(&self.prefix, asset).await
    }

    /// Sets multiple asset states for the specified query ids.
    pub async fn set_asset_infos(&self, assets: Vec<AssetInfo>) -> Result<(), S::Error> {
        self.store.insert_asset_infos(&self.prefix, assets).await
    }

    /// Gets multiple asset states for the specified query ids.
    pub async fn get_query_ids(&self) -> Result<HashSet<String>, S::Error> {
        let query_ids = self
            .store
            .get_query_ids(&self.prefix)
            .await?
            .unwrap_or_default();
        Ok(query_ids)
    }

    /// Calculates the [Difference] between the current query ids and a new set of query ids.
    pub async fn compute_query_id_difference(
        &self,
        ids: HashSet<String>,
    ) -> Result<Difference, S::Error> {
        let query_ids = self.get_query_ids().await?;
        let current_ids: HashSet<String, RandomState> = HashSet::from_iter(query_ids.into_iter());

        let added = ids
            .difference(&current_ids)
            .cloned()
            .collect::<Vec<String>>();
        let removed = current_ids
            .difference(&ids)
            .cloned()
            .collect::<Vec<String>>();

        Ok(Difference { added, removed })
    }

    /// Adds the specified query ids to the current set of query ids.
    pub async fn add_query_ids(&self, ids: Vec<String>) -> Result<(), S::Error> {
        if ids.is_empty() {
            return Ok(());
        }

        // Guard will be dropped at the end of this
        let _ = self.query_ids_lock.lock().await;
        let mut query_ids = self
            .store
            .get_query_ids(&self.prefix)
            .await?
            .unwrap_or_default();
        query_ids.extend(ids.into_iter());
        self.set_query_ids(query_ids).await
    }

    /// Removes the specified query ids from the current set of query ids.
    pub async fn remove_query_ids(&self, ids: &[String]) -> Result<(), S::Error> {
        if ids.is_empty() {
            return Ok(());
        }

        let _ = self.query_ids_lock.lock().await;
        let mut query_ids = self
            .store
            .get_query_ids(&self.prefix)
            .await?
            .unwrap_or_default();

        let curr_len = query_ids.len();
        for id in ids {
            query_ids.remove(id);
        }

        // Value is not overwritten if no changes are made to query id set
        if curr_len == query_ids.len() {
            Ok(())
        } else {
            self.set_query_ids(query_ids).await
        }
    }

    /// Completely overwrite the current query ids with the new set of query ids.
    pub async fn set_query_ids(&self, ids: HashSet<String>) -> Result<(), S::Error> {
        self.store.set_query_ids(&self.prefix, ids).await
    }
}

/// Contains the query ids that would be added and removed relative to the old set.
pub struct Difference {
    /// The query ids that would be added relative to the old set
    pub added: Vec<String>,
    /// The query ids that would be removed relative to the old set
    pub removed: Vec<String>,
}
