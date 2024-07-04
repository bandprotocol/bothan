use std::sync::Arc;

use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Mutex;

use asset_store::start_asset_store;
use bothan_core::store::{AssetStatus, AssetStore, Error as StoreError, Storage};

use crate::api::websocket::{BinanceWebSocketConnection, BinanceWebSocketConnector};

mod asset_store;
pub mod builder;
mod types;

/// A store that fetches and stores the asset information from Binance's API.
pub struct BinanceStore {
    connector: Arc<BinanceWebSocketConnector>,
    connection: Arc<Mutex<BinanceWebSocketConnection>>,
    storage: Arc<Storage>,
    internal_ch_size: usize,
    subscribe_ch_tx: Option<Sender<Vec<String>>>,
    unsubscribe_ch_tx: Option<Sender<Vec<String>>>,
}

impl BinanceStore {
    /// Create a new BinanceStore with the specified connector, connection, and internal channel size.
    pub fn new(
        connector: Arc<BinanceWebSocketConnector>,
        connection: Arc<Mutex<BinanceWebSocketConnection>>,
        internal_ch_size: usize,
    ) -> Self {
        Self {
            connector,
            connection,
            storage: Arc::new(Storage::default()),
            internal_ch_size,
            subscribe_ch_tx: None,
            unsubscribe_ch_tx: None,
        }
    }
}

#[async_trait::async_trait]
impl AssetStore for BinanceStore {
    /// Starts the BinanceStore service, setting up the necessary channels and spawning the asset store.
    async fn start(&mut self) {
        let (sub_tx, sub_rx) = channel(self.internal_ch_size);
        let (unsub_tx, unsub_rx) = channel(self.internal_ch_size);

        self.subscribe_ch_tx = Some(sub_tx);
        self.unsubscribe_ch_tx = Some(unsub_tx);

        let connector = self.connector.clone();
        let connection = self.connection.clone();
        let storage = self.storage.clone();

        tokio::spawn(async move {
            start_asset_store(connector, connection, storage, sub_rx, unsub_rx).await
        });
    }

    /// Fetches the AssetStatus for the given cryptocurrency ids.
    async fn get_assets(&self, ids: &[&str]) -> Vec<AssetStatus> {
        self.storage.get_assets(ids).await
    }

    /// Adds the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn add_query_ids(&mut self, ids: &[&str]) -> Result<(), StoreError> {
        let to_sub = self
            .storage
            .filter_existing_query_ids(ids.iter().map(|id| id.to_string()).collect::<Vec<String>>())
            .await;

        if let Some(tx) = &self.subscribe_ch_tx {
            if let Err(e) = tx.send(to_sub.clone()).await {
                Err(StoreError::ModifyQueryIDsFailed(e.to_string()))
            } else {
                self.storage.set_query_ids(to_sub).await;
                Ok(())
            }
        } else {
            Err(StoreError::NotStarted)
        }
    }

    /// Removes the specified cryptocurrency IDs to the query set and subscribes to their updates.
    async fn remove_query_ids(&mut self, ids: &[&str]) -> Result<(), StoreError> {
        let to_unsub = self
            .storage
            .filter_missing_query_ids(ids.iter().map(|id| id.to_string()).collect::<Vec<String>>())
            .await;

        if let Some(tx) = &self.unsubscribe_ch_tx {
            if let Err(e) = tx.send(to_unsub.clone()).await {
                Err(StoreError::ModifyQueryIDsFailed(e.to_string()))
            } else {
                self.storage.remove_query_ids(to_unsub.as_slice()).await;
                Ok(())
            }
        } else {
            Err(StoreError::NotStarted)
        }
    }
    /// Retrieves the current set of queried cryptocurrency IDs.
    async fn get_query_ids(&self) -> Vec<String> {
        self.storage.get_query_ids().await
    }
}
