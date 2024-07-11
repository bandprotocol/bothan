use std::sync::Arc;

use bothan_core::store::Store;
use bothan_core::worker::{AssetStatus, AssetWorker, Error};

use crate::api::CoinGeckoRestAPI;

mod asset_worker;
pub mod builder;
pub mod error;
pub mod opts;
pub mod types;

/// A worker that fetches and stores the asset information from CoinGecko's API.
pub struct CoinGeckoWorker {
    api: CoinGeckoRestAPI,
    store: Arc<Store>,
}

impl CoinGeckoWorker {
    /// Create a new worker with the specified api and store.
    pub fn new(api: CoinGeckoRestAPI, store: Arc<Store>) -> Self {
        Self { api, store }
    }
}

#[async_trait::async_trait]
impl AssetWorker for CoinGeckoWorker {
    /// Fetches the AssetStatus for the given cryptocurrency ids.
    async fn get_assets<K: AsRef<str> + Send + Sync>(&self, ids: &[K]) -> Vec<AssetStatus> {
        self.store.get_assets(ids).await
    }

    /// Adds the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn add_query_ids<K: Into<String> + Send + Sync>(&self, ids: Vec<K>) -> Result<(), Error> {
        self.store.add_query_ids(ids).await;
        Ok(())
    }

    /// Removes the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn remove_query_ids<K: AsRef<str> + Send + Sync>(&self, ids: &[K]) -> Result<(), Error> {
        self.store.remove_query_ids(ids).await;
        Ok(())
    }

    /// Retrieves the current set of queried cryptocurrency IDs.
    async fn get_query_ids(&self) -> Vec<String> {
        self.get_query_ids().await
    }
}
