use tokio::sync::mpsc::Sender;

use bothan_core::store::Store;
use bothan_core::worker::{AssetStatus, AssetWorker, Error};

use crate::api::websocket::BinanceWebSocketConnector;

mod asset_worker;
pub mod builder;
pub(crate) mod error;
mod types;

/// A store that fetches and stores the asset information from Binance's API.
pub struct BinanceWorker {
    connector: BinanceWebSocketConnector,
    store: Store,
    subscribe_tx: Sender<Vec<String>>,
    unsubscribe_tx: Sender<Vec<String>>,
}

impl BinanceWorker {
    /// Create a new BinanceStore with the specified connector, connection, and internal channel size.
    pub fn new(
        connector: BinanceWebSocketConnector,
        store: Store,
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
    async fn get_assets<T: AsRef<str> + Send + Sync>(&self, ids: &[T]) -> Vec<AssetStatus> {
        self.store.get_assets(ids).await
    }

    /// Adds the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn add_query_ids<T: AsRef<str> + Send + Sync>(&self, ids: &[T]) -> Result<(), Error> {
        let to_sub = self
            .store
            .filter_existing_query_ids(ids.iter().map(|id| id.as_ref()).collect::<Vec<&str>>())
            .await;

        if let Err(e) = self.subscribe_tx.send(to_sub.clone()).await {
            Err(Error::ModifyQueryIDsFailed(e.to_string()))
        } else {
            self.store.set_query_ids(to_sub).await;
            Ok(())
        }
    }

    /// Removes the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn remove_query_ids<T: AsRef<str> + Send + Sync>(&self, ids: &[T]) -> Result<(), Error> {
        let to_unsub = self
            .store
            .filter_missing_query_ids(ids.iter().map(|id| id.as_ref()).collect::<Vec<&str>>())
            .await;

        if let Err(e) = self.unsubscribe_tx.send(to_unsub.clone()).await {
            Err(Error::ModifyQueryIDsFailed(e.to_string()))
        } else {
            self.store.remove_query_ids(to_unsub.as_slice()).await;
            Ok(())
        }
    }
    /// Retrieves the current set of queried cryptocurrency IDs.
    async fn get_query_ids(&self) -> Vec<String> {
        self.store.get_query_ids().await
    }
}
