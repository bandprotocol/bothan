use tokio::sync::mpsc::Sender;

use bothan_core::store::WorkerStore;
use bothan_core::worker::{AssetState, AssetWorker, Error};

use crate::api::websocket::BinanceWebSocketConnector;

mod asset_worker;
pub mod builder;
pub(crate) mod error;
pub mod opts;
mod types;

/// A worker that fetches and stores the asset information from Binance's API.
pub struct BinanceWorker {
    connector: BinanceWebSocketConnector,
    store: WorkerStore,
    subscribe_tx: Sender<Vec<String>>,
    unsubscribe_tx: Sender<Vec<String>>,
}

impl BinanceWorker {
    /// Create a new worker with the specified connector, store and channels.
    pub fn new(
        connector: BinanceWebSocketConnector,
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
impl AssetWorker for BinanceWorker {
    /// Fetches the AssetStatus for the given cryptocurrency ids.
    async fn get_assets(&self, ids: &[&str]) -> Vec<AssetState> {
        self.store.get_assets(ids).await
    }

    /// Adds the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn add_query_ids(&self, ids: Vec<String>) -> Result<(), Error> {
        let to_sub = self.store.add_query_ids(ids).await;

        self.subscribe_tx
            .send(to_sub)
            .await
            .map_err(|e| Error::ModifyQueryIDsFailed(e.to_string()))
    }

    /// Removes the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn remove_query_ids(&self, ids: Vec<String>) -> Result<(), Error> {
        let to_unsub = self.store.remove_query_ids(ids).await;

        self.unsubscribe_tx
            .send(to_unsub.clone())
            .await
            .map_err(|e| Error::ModifyQueryIDsFailed(e.to_string()))
    }
}
