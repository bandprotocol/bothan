use bothan_core::store::error::Error as StoreError;
use bothan_core::store::WorkerStore;
use bothan_core::worker::{AssetState, AssetWorker, SetQueryIDError};

use crate::api::CoinMarketCapRestAPI;

mod asset_worker;
pub mod builder;
pub mod error;
pub mod opts;
pub mod types;

/// A worker that fetches and stores the asset information from CoinMarketCap's API.
pub struct CoinMarketCapWorker {
    api: CoinMarketCapRestAPI,
    store: WorkerStore,
}

impl CoinMarketCapWorker {
    /// Create a new worker with the specified api and store.
    pub fn new(api: CoinMarketCapRestAPI, store: WorkerStore) -> Self {
        Self { api, store }
    }
}

#[async_trait::async_trait]
impl AssetWorker for CoinMarketCapWorker {
    /// Fetches the AssetStatus for the given cryptocurrency ids.
    async fn get_asset(&self, id: &str) -> Result<AssetState, StoreError> {
        self.store.get_asset(&id).await
    }

    /// Adds the specified cryptocurrency IDs to the query set.
    async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), SetQueryIDError> {
        self.store
            .set_query_ids(ids)
            .await
            .map_err(|e| SetQueryIDError::new(e.to_string()))?;
        Ok(())
    }
}