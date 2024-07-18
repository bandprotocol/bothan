use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use tracing::error;

use bothan_core::store::Store;
use bothan_core::worker::{AssetStatus, AssetWorker, Error};

use crate::api::websocket::BinanceWebSocketConnector;

mod asset_worker;
pub mod builder;
pub(crate) mod error;
pub mod opts;
mod types;

/// A worker that fetches and stores the asset information from Binance's API.
pub struct BinanceWorker {
    connector: BinanceWebSocketConnector,
    store: Arc<Store>,
    subscribe_tx: Sender<Vec<String>>,
    unsubscribe_tx: Sender<Vec<String>>,
}

impl BinanceWorker {
    /// Create a new worker with the specified connector, store and channels.
    pub fn new(
        connector: BinanceWebSocketConnector,
        store: Arc<Store>,
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
    async fn get_assets(&self, ids: &[&str]) -> Vec<AssetStatus> {
        self.store.get_assets(ids).await
    }

    /// Adds the specified cryptocurrency IDs to the query set.
    async fn add_query_ids(&self, ids: Vec<String>) -> Result<(), Error> {
        let to_sub = self.store.add_query_ids(ids).await;

        if let Err(e) = self.subscribe_tx.send(to_sub.clone()).await {
            error!("failed to add query ids: {}", e);
            self.store.remove_query_ids(to_sub.as_slice()).await;
            Err(Error::ModifyQueryIDsFailed(e.to_string()))
        } else {
            Ok(())
        }
    }

    /// Removes the specified cryptocurrency IDs from the query set.
    async fn remove_query_ids(&self, ids: &[&str]) -> Result<(), Error> {
        let to_unsub = self.store.remove_query_ids(ids).await;

        if let Err(e) = self.unsubscribe_tx.send(to_unsub.clone()).await {
            error!("failed to remove query ids: {}", e);
            self.store.add_query_ids(to_unsub).await;
            Err(Error::ModifyQueryIDsFailed(e.to_string()))
        } else {
            Ok(())
        }
    }

    /// Retrieves the current set of queried cryptocurrency IDs.
    async fn get_query_ids(&self) -> Vec<String> {
        self.store.get_query_ids().await
    }
}
