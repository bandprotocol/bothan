use std::collections::HashSet;

use crate::store::error::Error;
use crate::store::{QueryIDs, SharedStore};
use crate::types::AssetInfo;
use crate::worker::AssetState;

pub struct WorkerStore {
    store: SharedStore,
    prefix: String,
}

impl WorkerStore {
    pub fn new<T: Into<String>>(store: SharedStore, prefix: T) -> Self {
        Self {
            store,
            prefix: prefix.into(),
        }
    }

    pub async fn get_asset<K: AsRef<str>>(&self, id: &K) -> Result<AssetState, Error> {
        if !self.store.contains_query_id(&self.prefix, id).await? {
            return Ok(AssetState::Unsupported);
        }

        match self.store.get_asset_info(&self.prefix, id).await? {
            Some(asset) => Ok(AssetState::Available(asset)),
            None => Ok(AssetState::Pending),
        }
    }

    pub async fn set_asset<K: AsRef<str>>(
        &self,
        id: K,
        asset_info: AssetInfo,
    ) -> Result<(), Error> {
        self.store
            .insert_asset_info(&self.prefix, &id, asset_info)
            .await
    }

    pub async fn set_assets<K: AsRef<str>>(
        &self,
        assets: Vec<(K, AssetInfo)>,
    ) -> Result<(), Error> {
        self.store.insert_asset_infos(&self.prefix, assets).await
    }

    // TODO: Deprecate when the new query_id system is in place
    pub async fn set_query_ids<K>(&self, ids: Vec<K>) -> Result<(Vec<String>, Vec<String>), Error>
    where
        K: Into<String> + Clone,
    {
        let current_ids = self.get_query_ids().await?;
        let new_ids: QueryIDs = HashSet::from_iter(ids.into_iter().map(Into::into));

        let added = new_ids
            .difference(&current_ids)
            .cloned()
            .collect::<Vec<String>>();
        let removed = current_ids
            .difference(&new_ids)
            .cloned()
            .collect::<Vec<String>>();

        self.store.set_query_ids(&self.prefix, new_ids).await?;

        Ok((added, removed))
    }

    // Computes the signals to add and remove from the query set if the given IDs is to replace
    // the current query_id set
    pub async fn compute_query_id_differences<K>(
        &self,
        ids: Vec<K>,
    ) -> Result<(Vec<String>, Vec<String>), Error>
    where
        K: Into<String> + Clone,
    {
        let current_ids = self.get_query_ids().await?;
        let new_ids: QueryIDs = HashSet::from_iter(ids.into_iter().map(Into::into));

        let to_add = new_ids
            .difference(&current_ids)
            .cloned()
            .collect::<Vec<String>>();
        let to_remove = current_ids
            .difference(&new_ids)
            .cloned()
            .collect::<Vec<String>>();

        Ok((to_add, to_remove))
    }

    pub async fn add_query_ids<K>(&self, ids: Vec<K>) -> Result<Vec<K>, Error>
    where
        K: Into<String> + Clone,
    {
        let changes = self
            .store
            .insert_query_ids(&self.prefix, ids.clone())
            .await?;

        let added = ids
            .into_iter()
            .zip(changes.into_iter())
            .filter(|(_, changed)| *changed)
            .map(|(id, _)| id)
            .collect();
        Ok(added)
    }

    pub async fn remove_query_ids<K>(&self, ids: Vec<K>) -> Result<Vec<K>, Error>
    where
        K: Into<String> + AsRef<str>,
    {
        let changes = self
            .store
            .remove_query_ids(&self.prefix, ids.as_slice())
            .await?;

        let removed = ids
            .into_iter()
            .zip(changes.into_iter())
            .filter(|(_, changed)| *changed)
            .map(|(id, _)| id)
            .collect();
        Ok(removed)
    }

    pub async fn get_query_ids(&self) -> Result<QueryIDs, Error> {
        let query_ids = self
            .store
            .get_query_ids(&self.prefix)
            .await?
            .unwrap_or_default();
        Ok(query_ids)
    }
}
