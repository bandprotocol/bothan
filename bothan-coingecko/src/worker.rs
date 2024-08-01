use bothan_core::store::WorkerStore;
use bothan_core::worker::{AssetState, AssetWorker, Error};

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
    async fn get_assets(&self, ids: &[&str]) -> Vec<AssetState> {
        self.store.get_assets(ids).await
    }

    /// Adds the specified cryptocurrency IDs to the query set.
    async fn add_query_ids(&self, ids: Vec<String>) -> Result<(), Error> {
        self.store.add_query_ids(ids).await;
        Ok(())
    }

    /// Removes the specified cryptocurrency IDs from the query set.
    async fn remove_query_ids(&self, ids: Vec<String>) -> Result<(), Error> {
        self.store.remove_query_ids(ids).await;
        Ok(())
    }
}
