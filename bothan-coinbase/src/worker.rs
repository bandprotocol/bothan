use tokio::sync::mpsc::Sender;

use bothan_core::store::error::Error as StoreError;
use bothan_core::store::WorkerStore;
use bothan_core::worker::{AssetState, AssetWorker, SetQueryIDError};

use crate::api::websocket::CoinbaseWebSocketConnector;

mod asset_worker;
pub mod builder;
pub(crate) mod error;
pub mod opts;
mod types;

/// A worker that fetches and stores the asset information from Coinbase's API.
pub struct CoinbaseWorker {
    connector: CoinbaseWebSocketConnector,
    store: WorkerStore,
    subscribe_tx: Sender<Vec<String>>,
    unsubscribe_tx: Sender<Vec<String>>,
}

impl CoinbaseWorker {
    /// Create a new worker with the specified connector, store and channels.
    pub fn new(
        connector: CoinbaseWebSocketConnector,
        store: WorkerStore,
        subscribe_tx: Sender<Vec<String>>,
        unsubscribe_tx: Sender<Vec<String>>,
    ) -> Self {
        Self {
            connector,
            store,
            subscribe_tx,
            unsubscribe_tx,
        }
    }
}

#[async_trait::async_trait]
impl AssetWorker for CoinbaseWorker {
    /// Fetches the AssetStatus for the given cryptocurrency id.
    async fn get_asset(&self, id: &str) -> Result<AssetState, StoreError> {
        self.store.get_asset(&id).await
    }

    /// Sets the specified cryptocurrency IDs to the query. If the ids are already in the query set,
    /// it will not be resubscribed.
    async fn set_query_ids(&self, ids: Vec<String>) -> Result<(), SetQueryIDError> {
        let (to_sub, to_unsub) = self
            .store
            .set_query_ids(ids)
            .await
            .map_err(|e| SetQueryIDError::new(e.to_string()))?;

        self.subscribe_tx
            .send(to_sub)
            .await
            .map_err(|e| SetQueryIDError::new(e.to_string()))?;

        self.unsubscribe_tx
            .send(to_unsub)
            .await
            .map_err(|e| SetQueryIDError::new(e.to_string()))?;

        Ok(())
    }
}
