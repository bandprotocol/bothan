use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::types::AssetInfo;

use crate::api::error::Error as BinanceError;
use crate::api::types::{BinanceResponse, Data};
use crate::api::websocket::{BinanceWebSocketConnection, BinanceWebSocketConnector};
use crate::store::types::RECONNECT_BUFFER;
use crate::types::DEFAULT_TIMEOUT;

pub async fn start_asset_store(
    connector: Arc<BinanceWebSocketConnector>,
    connection: Arc<Mutex<BinanceWebSocketConnection>>,
    data_store: Arc<RwLock<HashMap<String, AssetInfo>>>,
    query_ids: Arc<RwLock<HashSet<String>>>,
    mut subscribe_ch_rx: Receiver<Vec<String>>,
    mut unsubscribe_ch_rx: Receiver<Vec<String>>,
) {
    loop {
        select! {
            id = subscribe_ch_rx.recv() => {
                handle_subscribe_recv(id, &connection).await;
            },
            id = unsubscribe_ch_rx.recv() => {
                handle_unsubscribe_recv(id, &connection).await;
            },
            result = {
                let cloned = connection.clone();
                // TODO: Need to find a better way to resolve this
                timeout(DEFAULT_TIMEOUT, async move { cloned.lock().await.next().await })
            } => {
                match result {
                    Ok(Err(BinanceError::UnsupportedMessage)) | Err(_) => handle_reconnect(&connector, &connection, &query_ids).await,
                    Ok(binance_result) => handle_connection_recv(binance_result, &data_store).await,
                }
            },

        }
    }
}

async fn subscribe(
    ids: Vec<String>,
    connection: &Mutex<BinanceWebSocketConnection>,
) -> Result<(), anyhow::Error> {
    if !ids.is_empty() {
        connection
            .lock()
            .await
            .subscribe(&ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_subscribe_recv(
    ids: Option<Vec<String>>,
    connection: &Mutex<BinanceWebSocketConnection>,
) {
    match ids {
        Some(ids) => {
            if let Err(e) = subscribe(ids, connection).await {
                error!("failed to subscribe: {}", e);
            } else {
                info!("subscribed to ids");
            }
        }
        None => {
            panic!("subscribe channel closed")
        }
    }
}

async fn handle_unsubscribe_recv(
    ids: Option<Vec<String>>,
    connection: &Mutex<BinanceWebSocketConnection>,
) {
    match ids {
        Some(ids) => {
            if ids.is_empty() {
                debug!("received empty unsubscribe command");
            } else {
                let res = connection
                    .lock()
                    .await
                    .unsubscribe(&ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
                    .await;
                if res.is_err() {
                    error!("failed to unsubscribe to ids: {:?}", ids);
                } else {
                    info!("unsubscribed to ids: {:?}", ids);
                }
            }
        }
        None => {
            panic!("unsubscribe channel closed")
        }
    }
}

async fn handle_reconnect(
    connector: &BinanceWebSocketConnector,
    connection: &Mutex<BinanceWebSocketConnection>,
    query_ids: &RwLock<HashSet<String>>,
) {
    loop {
        // Attempt to reconnect to binance
        let mut locked = connection.lock().await;
        warn!("attempting to reconnect to binance");

        if let Ok(new_connection) = connector.connect().await {
            *locked = new_connection;

            // Resubscribe to all ids
            let ids = query_ids.read().await.iter().cloned().collect();
            match subscribe(ids, connection).await {
                Ok(_) => {
                    info!("resubscribed to all ids");
                    return;
                }
                Err(_) => {
                    error!("failed to resubscribe to all ids");
                }
            }
        } else {
            error!("failed to reconnect to binance");
        }

        error!("retrying reconnect process");
        sleep(RECONNECT_BUFFER).await;
    }
}

async fn save_data(
    data: Data,
    data_store: &RwLock<HashMap<String, AssetInfo>>,
) -> anyhow::Result<()> {
    match data {
        Data::MiniTicker(ticker) => {
            let mut writer = data_store.write().await;
            let id = ticker.symbol.to_lowercase();
            let price_data = AssetInfo {
                id: id.clone(),
                price: Decimal::from_str_exact(&ticker.close_price)?,
                timestamp: ticker.event_time,
            };

            writer.insert(id, price_data);
        }
    }

    Ok(())
}

async fn process_response(resp: BinanceResponse, data_store: &RwLock<HashMap<String, AssetInfo>>) {
    match resp {
        BinanceResponse::Stream(resp) => match save_data(resp.data, data_store).await {
            Ok(_) => info!("saved data"),
            Err(e) => error!("failed to save data: {}", e),
        },
        BinanceResponse::Success(_) => {
            info!("subscription success");
        }
        BinanceResponse::Ping => {
            debug!("received ping from binance");
        }
        BinanceResponse::Error(e) => {
            error!("error code {} received from binance: {}", e.code, e.msg);
        }
    }
}

async fn handle_connection_recv(
    recv_result: Result<BinanceResponse, BinanceError>,
    data_store: &RwLock<HashMap<String, AssetInfo>>,
) {
    match recv_result {
        Ok(resp) => {
            process_response(resp, data_store).await;
        }
        Err(BinanceError::UnsupportedMessage) => {
            error!("unsupported message received from binance");
        }
        Err(e) => {
            panic!("{} should never be received", e)
        }
    }
}
