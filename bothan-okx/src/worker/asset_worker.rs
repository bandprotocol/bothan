use std::sync::Weak;
use std::time::Duration;

use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::types::AssetInfo;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use crate::api::error::{MessageError, SendError};
use crate::api::types::{ChannelResponse, OkxResponse, TickerData};
use crate::api::{WebSocketConnection, WebsocketConnector};
use crate::worker::InnerWorker;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
const RECONNECT_BUFFER: Duration = Duration::from_secs(5);

pub(crate) async fn start_asset_worker<S: Store>(
    worker: Weak<InnerWorker<S>>,
    mut connection: WebSocketConnection,
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
                        Ok(okx_result) => handle_connection_recv(okx_result, &worker.connector, &mut connection, &worker.store).await,
                    }
                } else {
                    break
                }
            }
        }
    }

    // Close the connection upon exiting
    if let Err(e) = connection.close().await {
        error!("asset worker failed to send close frame: {}", e)
    } else {
        debug!("asset worker successfully sent close frame")
    }

    debug!("asset worker has been dropped, stopping asset worker");
}

async fn subscribe(ids: &[String], connection: &mut WebSocketConnection) -> Result<(), SendError> {
    if !ids.is_empty() {
        let ids_vec = ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        connection.subscribe_ticker(&ids_vec).await?
    }

    Ok(())
}

async fn handle_subscribe_recv(ids: Vec<String>, connection: &mut WebSocketConnection) {
    if let Err(e) = subscribe(&ids, connection).await {
        error!("failed to subscribe to ids {:?}: {}", ids, e);
    } else {
        info!("subscribed to ids {:?}", ids);
    }
}

async fn unsubscribe(
    ids: &[String],
    connection: &mut WebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .unsubscribe_ticker(&ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_unsubscribe_recv(ids: Vec<String>, connection: &mut WebSocketConnection) {
    if let Err(e) = unsubscribe(&ids, connection).await {
        error!("failed to unsubscribe to ids {:?}: {}", ids, e);
    } else {
        info!("unsubscribed to ids {:?}", ids);
    }
}

async fn handle_reconnect<S: Store>(
    connector: &WebsocketConnector,
    connection: &mut WebSocketConnection,
    query_ids: &WorkerStore<S>,
) {
    let mut retry_count: usize = 1;
    loop {
        warn!("reconnecting: attempt {}", retry_count);

        if let Ok(new_connection) = connector.connect().await {
            *connection = new_connection;

            // Resubscribe to all ids
            let Ok(ids) = query_ids.get_query_ids().await else {
                error!("failed to get query ids from store");
                return;
            };

            let ids_vec = ids.into_iter().collect::<Vec<String>>();

            match subscribe(&ids_vec, connection).await {
                Ok(_) => {
                    info!("resubscribed to all ids");
                    return;
                }
                Err(_) => {
                    error!("failed to resubscribe to all ids");
                }
            }
        } else {
            error!("failed to reconnect to okx");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

async fn store_ticker<S: Store>(store: &WorkerStore<S>, ticker: TickerData, timestamp: i64) {
    let id = ticker.inst_id.clone();
    match Decimal::from_str_exact(&ticker.last) {
        Ok(price) => {
            let asset_info = AssetInfo::new(id.clone(), price, timestamp);
            if let Err(e) = store.set_asset_info(asset_info).await {
                error!("failed to store data for id {}: {}", id, e);
            } else {
                debug!("stored data for id {}", id);
            }
        }
        Err(e) => {
            error!("failed to parse price for id {}: {}", id, e);
        }
    }
}

async fn process_response<S: Store>(resp: OkxResponse, store: &WorkerStore<S>) {
    match resp {
        OkxResponse::ChannelResponse(resp) => match resp {
            ChannelResponse::Ticker(push_data) => {
                let timestamp = chrono::Utc::now().timestamp();
                for ticker in push_data.data {
                    store_ticker(store, ticker, timestamp).await
                }
            }
        },
        OkxResponse::WebSocketMessageResponse(resp) => {
            debug!("received public message from okx: {:?}", resp);
        }
    }
}

async fn handle_connection_recv<S: Store>(
    recv_result: Result<OkxResponse, MessageError>,
    connector: &WebsocketConnector,
    connection: &mut WebSocketConnection,
    store: &WorkerStore<S>,
) {
    match recv_result {
        Ok(resp) => {
            process_response(resp, store).await;
        }
        Err(MessageError::ChannelClosed) => {
            handle_reconnect(connector, connection, store).await;
        }
        Err(MessageError::UnsupportedMessage) => {
            error!("unsupported message received from okx");
        }
        Err(MessageError::Parse(..)) => {
            error!("unable to parse message from okx");
        }
    }
}
