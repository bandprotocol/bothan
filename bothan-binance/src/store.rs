use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::mpsc::{channel, Sender};
use tokio::sync::Mutex;
use tokio::sync::RwLock;

use asset_store::start_asset_store;
use bothan_core::store::{AssetStatus, AssetStore, Error as StoreError};
use bothan_core::types::AssetInfo;

use crate::api::websocket::{BinanceWebSocketConnection, BinanceWebSocketConnector};

mod asset_store;
pub mod builder;
mod types;

/// A service that fetches and caches the cryptocurrency prices from Binance.
pub struct BinanceStore {
    connector: Arc<BinanceWebSocketConnector>,
    connection: Arc<Mutex<BinanceWebSocketConnection>>,
    data_store: Arc<RwLock<HashMap<String, AssetInfo>>>,
    query_ids: Arc<RwLock<HashSet<String>>>,
    internal_ch_size: usize,
    subscribe_ch_tx: Option<Sender<Vec<String>>>,
    unsubscribe_ch_tx: Option<Sender<Vec<String>>>,
}

impl BinanceStore {
    /// Create a new Binance service with the connector, connection, and channel sizes.
    pub fn new(
        connector: Arc<BinanceWebSocketConnector>,
        connection: Arc<Mutex<BinanceWebSocketConnection>>,
        internal_ch_size: usize,
    ) -> Self {
        Self {
            connector,
            connection,
            data_store: Arc::new(RwLock::new(HashMap::new())),
            query_ids: Arc::new(RwLock::new(HashSet::new())),
            internal_ch_size,
            subscribe_ch_tx: None,
            unsubscribe_ch_tx: None,
        }
    }
}

#[async_trait::async_trait]
impl AssetStore for BinanceStore {
    async fn start(&mut self) {
        let (sub_tx, sub_rx) = channel(self.internal_ch_size);
        let (unsub_tx, unsub_rx) = channel(self.internal_ch_size);

        self.subscribe_ch_tx = Some(sub_tx);
        self.unsubscribe_ch_tx = Some(unsub_tx);

        let connector = self.connector.clone();
        let connection = self.connection.clone();
        let data_store = self.data_store.clone();
        let query_ids = self.query_ids.clone();

        tokio::spawn(async move {
            start_asset_store(
                connector, connection, data_store, query_ids, sub_rx, unsub_rx,
            )
            .await
        });
    }

    /// Fetches the price data for the given cryptocurrency ids.
    async fn get_assets(&self, ids: &[&str]) -> Vec<AssetStatus> {
        let data_reader = self.data_store.read().await;
        let id_reader = self.query_ids.read().await;
        ids.iter()
            .map(|id| {
                if !id_reader.contains(*id) {
                    return AssetStatus::Unsupported;
                }

                match data_reader.get(*id) {
                    Some(asset_info) => AssetStatus::Available(asset_info.clone()),
                    None => AssetStatus::Pending,
                }
            })
            .collect()
    }

    async fn add_query_ids(&mut self, ids: &[&str]) -> Result<(), StoreError> {
        let mut writer = self.query_ids.write().await;
        let sub = ids
            .iter()
            .filter_map(|id| {
                if writer.insert(id.to_string()) {
                    Some(id.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        if let Some(tx) = &self.subscribe_ch_tx {
            if let Err(e) = tx.send(sub).await {
                Err(StoreError::ModifyQueryIDsFailed(e.to_string()))
            } else {
                Ok(())
            }
        } else {
            Err(StoreError::NotStarted)
        }
    }

    async fn remove_query_ids(&mut self, ids: &[&str]) -> Result<(), StoreError> {
        let mut writer = self.query_ids.write().await;
        let unsub = ids
            .iter()
            .filter_map(|id| writer.take(*id))
            .collect::<Vec<String>>();

        if let Some(tx) = &self.unsubscribe_ch_tx {
            if let Err(e) = tx.send(unsub).await {
                Err(StoreError::ModifyQueryIDsFailed(e.to_string()))
            } else {
                Ok(())
            }
        } else {
            Err(StoreError::NotStarted)
        }
    }

    async fn get_query_ids(&self) -> Vec<String> {
        let reader = self.query_ids.read().await;
        reader.iter().cloned().collect()
    }
}

// #[cfg(test)]
// mod test {
//     use tokio_tungstenite::tungstenite::Message;
//     use ws_mock::ws_mock_server::WsMock;
//
//     use crate::api::types::{Data, MiniTickerInfo, StreamResponse};
//     use crate::api::websocket::test::setup_mock_server;
//
//     use super::*;
//
//     #[tokio::test]
//     async fn test_process_command() {
//         let mock = setup_mock_server().await;
//
//         let connector = Arc::new(BinanceWebSocketConnector::new(mock.uri().await));
//         let connection = Arc::new(Mutex::new(connector.connect().await.unwrap()));
//
//         let cache = Arc::new(Cache::new(None));
//
//         let (_, msg_rx) = channel::<Message>(32);
//
//     }
//
//     #[tokio::test]
//     async fn test_save_datum() {
//         let cache = Arc::new(Cache::new(None));
//         cache.set_pending("btcusdt".to_string()).await;
//
//         let ticker = MiniTickerInfo {
//             event_time: 1628794647025,
//             symbol: "BTCUSDT".to_string(),
//             close_price: "45000.00".to_string(),
//             open_price: "44000.00".to_string(),
//             high_price: "46000.00".to_string(),
//             low_price: "43000.00".to_string(),
//             base_volume: "1000.00".to_string(),
//             quote_volume: "45000000.00".to_string(),
//         };
//
//         let data = Data::MiniTicker(ticker);
//         save_datum(&data, &cache).await;
//
//         let price_data = cache.get("btcusdt").await.unwrap();
//         assert_eq!(
//             price_data,
//             AssetInfo {
//                 id: "btcusdt".to_string(),
//                 price: "45000.00".to_string(),
//                 timestamp: 1628794647025,
//             }
//         );
//     }
//
//     #[tokio::test]
//     async fn test_process_response() {
//         let cache = Arc::new(Cache::new(None));
//         cache.set_pending("btcusdt".to_string()).await;
//
//         let ticker = MiniTickerInfo {
//             event_time: 1628794647025,
//             symbol: "BTCUSDT".to_string(),
//             close_price: "45000.00".to_string(),
//             open_price: "44000.00".to_string(),
//             high_price: "46000.00".to_string(),
//             low_price: "43000.00".to_string(),
//             base_volume: "1000.00".to_string(),
//             quote_volume: "45000000.00".to_string(),
//         };
//
//         let data = Data::MiniTicker(ticker);
//         let stream_resp = StreamResponse {
//             stream: "btcusdt@miniTicker".to_string(),
//             data,
//         };
//
//         let response = BinanceResponse::Stream(stream_resp);
//         process_response(&response, &cache).await;
//
//         let price_data = cache.get("btcusdt").await.unwrap();
//         assert_eq!(
//             price_data,
//             AssetInfo {
//                 id: "btcusdt".to_string(),
//                 price: "45000.00".to_string(),
//                 timestamp: 1628794647025,
//             }
//         );
//     }
// }
