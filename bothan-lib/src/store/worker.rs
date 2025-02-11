use crate::store::Store;
use crate::types::AssetInfo;
use crate::types::AssetState;
use std::collections::HashSet;
use std::hash::RandomState;

#[derive(Clone)]
pub struct WorkerStore<S: Store> {
    store: S,
    prefix: String,
}

impl<S: Store> WorkerStore<S> {
    pub fn new<T: Into<String>>(store: &S, prefix: T) -> Self {
        Self {
            store: store.clone(),
            prefix: prefix.into(),
        }
    }

    pub async fn get_asset(&self, id: &str) -> Result<AssetState, S::Error> {
        if !self.store.contains_query_id(&self.prefix, id).await? {
            return Ok(AssetState::Unsupported);
        }

        match self.store.get_asset_info(&self.prefix, id).await? {
            Some(asset) => Ok(AssetState::Available(asset)),
            None => Ok(AssetState::Pending),
        }
    }

    pub async fn set_asset(&self, id: String, asset_info: AssetInfo) -> Result<(), S::Error> {
        self.store
            .insert_asset_info(&self.prefix, (id, asset_info))
            .await
    }

    pub async fn set_assets(&self, assets: Vec<(String, AssetInfo)>) -> Result<(), S::Error> {
        self.store
            .insert_asset_info_batch(&self.prefix, assets)
            .await
    }

    pub async fn get_query_ids(&self) -> Result<Vec<String>, S::Error> {
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
        ids: Vec<String>,
    ) -> Result<Difference, S::Error> {
        let query_ids = self.get_query_ids().await?;
        let current_ids: HashSet<String, RandomState> = HashSet::from_iter(query_ids.into_iter());
        let new_ids = HashSet::from_iter(ids.into_iter());

        let added = new_ids
            .difference(&current_ids)
            .cloned()
            .collect::<Vec<String>>();
        let removed = current_ids
            .difference(&new_ids)
            .cloned()
            .collect::<Vec<String>>();

        Ok(Difference { added, removed })
    }

    /// Adds the specified query ids to the current set of query ids.
    pub async fn add_query_ids(&self, ids: Vec<String>) -> Result<(), S::Error> {
        self.store.insert_query_ids(&self.prefix, ids).await
    }

    /// Removes the specified query ids from the current set of query ids.
    pub async fn remove_query_ids(&self, ids: &[String]) -> Result<(), S::Error> {
        self.store.remove_query_ids(&self.prefix, ids).await
    }

    /// Completely overwrite the current query ids with the new set of query ids.
    pub async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), S::Error> {
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
