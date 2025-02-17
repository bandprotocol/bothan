use std::sync::Weak;

use crate::api::error::{MessageError, SendError};
use crate::api::types::{HtxResponse, Pong, Tick};
use crate::api::{HtxWebSocketConnection, HtxWebSocketConnector};
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::InnerWorker;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::types::AssetInfo;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

pub(crate) async fn start_asset_worker<S: Store>(
    worker: Weak<InnerWorker<S>>,
    mut connection: HtxWebSocketConnection,
    mut subscribe_rx: Receiver<Vec<String>>,
    mut unsubscribe_rx: Receiver<Vec<String>>,
) {
    loop {
        select! {
            Some(ids) = subscribe_rx.recv() => handle_subscribe_recv(ids, &mut connection).await,
            Some(ids) = unsubscribe_rx.recv() => handle_unsubscribe_recv(ids, &mut connection).await,
            result = timeout(DEFAULT_TIMEOUT, connection.next()) => {
                if let Some(worker) = worker.upgrade() {
                    match result {
                        Err(_) => handle_reconnect(&worker.connector, &mut connection, &worker.store).await,
                        Ok(htx_result) => handle_connection_recv(htx_result, &worker.connector, &mut connection, &worker.store).await,
                    }
                } else {
                    break
                }
            }
        }
    }

    // Close the connection upon exiting
    if let Err(e) = connection.close().await {
        error!("asset worker failed to send close frame: {}", e);
    } else {
        debug!("asset worker successfully sent close frame");
    }

    debug!("asset worker has been dropped, stopping asset worker");
}

/// Handles subscription to multiple ids by subscribing one at a time.
async fn handle_subscribe_recv(ids: Vec<String>, connection: &mut HtxWebSocketConnection) {
    for id in ids {
        if let Err(e) = subscribe(&id, connection).await {
            error!("failed to subscribe to id {}: {}", id, e);
        } else {
            info!("subscribed to id {}", id);
        }
    }
}

/// Subscribes to a single id on the WebSocket connection.
async fn subscribe(id: &str, connection: &mut HtxWebSocketConnection) -> Result<(), SendError> {
    connection.subscribe_ticker(id).await
}

/// Handles unsubscription from multiple ids by unsubscribing one at a time.
async fn handle_unsubscribe_recv(ids: Vec<String>, connection: &mut HtxWebSocketConnection) {
    for id in ids {
        if let Err(e) = unsubscribe(&id, connection).await {
            error!("failed to unsubscribe from id {}: {}", id, e);
        } else {
            info!("unsubscribed from id {}", id);
        }
    }
}

/// Unsubscribes from a single id on the WebSocket connection.
async fn unsubscribe(id: &str, connection: &mut HtxWebSocketConnection) -> Result<(), SendError> {
    connection.unsubscribe_ticker(id).await
}

/// Handles reconnection to the WebSocket and re-subscribes to all previously subscribed ids.
async fn handle_reconnect<S: Store>(
    connector: &HtxWebSocketConnector,
    connection: &mut HtxWebSocketConnection,
    store: &WorkerStore<S>,
) {
    let mut retry_count: usize = 1;
    loop {
        warn!("reconnecting: attempt {}", retry_count);

        if let Ok(new_connection) = connector.connect().await {
            *connection = new_connection;

            // Resubscribe to all ids
            let Ok(ids) = store.get_query_ids().await else {
                error!("failed to get query ids from store");
                return;
            };

            for id in ids {
                if let Err(e) = subscribe(&id, connection).await {
                    error!("failed to resubscribe to id {}: {}", id, e);
                } else {
                    info!("resubscribed to id {}", id);
                }
            }
            return;
        } else {
            error!("failed to reconnect to htx");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

/// Stores tick information into the worker store.
async fn store_tick<S: Store, T: Into<String>>(store: &WorkerStore<S>, id: T, tick: Tick) {
    let id = id.into();
    match Decimal::from_f64(tick.last_price) {
        Some(price) => {
            let asset_info = AssetInfo::new(id.clone(), price, 0);
            if let Err(e) = store.set_asset(id.clone(), asset_info).await {
                error!("failed to store data for id {}: {}", id, e);
            }
            debug!("stored data for id {}", id);
        }
        None => {
            error!("data for id {} has a nan price", id);
        }
    }
}

/// Processes the response from the Htx API and handles each type accordingly.
async fn process_response<S: Store>(
    resp: HtxResponse,
    store: &WorkerStore<S>,
    connection: &mut HtxWebSocketConnection,
) {
    match resp {
        HtxResponse::SubResponse(resp) => {
            debug!("received subscribe response from htx: {:?}", resp);
        }
        HtxResponse::UnsubResponse(resp) => {
            debug!("received unsubscribe response from htx: {:?}", resp);
        }
        HtxResponse::DataUpdate(data) => {
            debug!("received data update from channel {}", data.ch);
            if let Some(id) = data.ch.split('.').nth(1) {
                // Handle processing of data update, e.g., storing tick data
                store_tick(store, id, data.tick).await
            }
        }
        HtxResponse::Ping(ping) => {
            debug!("received ping from htx: {}", ping.ping);
            // Send a pong response back
            if let Err(e) = send_pong(ping.ping, connection).await {
                error!("failed to send pong: {}", e);
            } else {
                debug!("sent pong in response to ping: {}", ping.ping);
            }
        }
    }
}

/// Sends a pong response back to the WebSocket connection.
async fn send_pong(ping: u64, connection: &mut HtxWebSocketConnection) -> Result<(), SendError> {
    let pong_payload = Pong { pong: ping }; // Create the Pong struct with the ping timestamp
    connection.send_pong(pong_payload).await // Assuming send_pong is implemented to send Pong struct
}

/// Handles received messages from the WebSocket connection.
async fn handle_connection_recv<S: Store>(
    recv_result: Result<HtxResponse, MessageError>,
    connector: &HtxWebSocketConnector,
    connection: &mut HtxWebSocketConnection,
    store: &WorkerStore<S>,
) {
    match recv_result {
        Ok(resp) => {
            process_response(resp, store, connection).await;
        }
        Err(MessageError::ChannelClosed) => {
            handle_reconnect(connector, connection, store).await;
        }
        Err(MessageError::UnsupportedMessage) => {
            error!("unsupported message received from htx");
        }
        Err(MessageError::Parse(..)) => {
            error!("unable to parse message from htx");
        }
    }
}
