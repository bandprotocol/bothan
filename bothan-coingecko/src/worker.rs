use bothan_core::store::errors::Error as StoreError;
use bothan_core::store::WorkerStore;
use bothan_core::worker::{AssetState, AssetWorker, SetQueryIDError};

use crate::api::CoinGeckoRestAPI;

mod asset_worker;
pub mod builder;
pub mod error;
pub mod opts;
pub mod types;

/// A worker that fetches and stores the asset information from CoinGecko's API.
pub struct CoinGeckoWorker {
    api: CoinGeckoRestAPI,
    store: WorkerStore,
}

impl CoinGeckoWorker {
    /// Create a new worker with the specified api and store.
    pub fn new(api: CoinGeckoRestAPI, store: WorkerStore) -> Self {
        Self { api, store }
    }
}

#[async_trait::async_trait]
impl AssetWorker for CoinGeckoWorker {
    /// Fetches the AssetStatus for the given cryptocurrency ids.
    async fn get_asset(&self, id: &str) -> Result<AssetState, StoreError> {
        self.store.get_asset(&id).await
    }

    /// Adds the specified cryptocurrency IDs to the query set.
    async fn add_query_ids(&self, ids: Vec<String>) -> Result<(), SetQueryIDError> {
        self.store
            .add_query_ids(ids)
            .await
            .map_err(|e| SetQueryIDError::new(e.to_string()))?;
        Ok(())
    }

    /// Removes the specified cryptocurrency IDs from the query set.
    async fn remove_query_ids(&self, ids: Vec<String>) -> Result<(), SetQueryIDError> {
        self.store
            .remove_query_ids(ids)
            .await
            .map_err(|e| SetQueryIDError::new(e.to_string()))?;
        Ok(())
    }
}
