use std::sync::Weak;

use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::error::{MessageError, SendError};
use crate::api::msgs::{BinanceResponse, Data};
use crate::api::{BinanceWebSocketConnection, BinanceWebSocketConnector};
use crate::worker::error::ParseError;
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::BinanceWorker;

pub(crate) async fn start_asset_worker(
    worker: Weak<BinanceWorker>,
    mut connection: BinanceWebSocketConnection,
    mut subscribe_rx: Receiver<Vec<String>>,
    mut unsubscribe_rx: Receiver<Vec<String>>,
) {
    while let Some(worker) = worker.upgrade() {
        select! {
            Some(ids) = subscribe_rx.recv() => handle_subscribe_recv(ids, &worker.store, &mut connection).await,
            Some(ids) = unsubscribe_rx.recv() => handle_unsubscribe_recv(ids, &worker.store, &mut connection).await,
            result = timeout(DEFAULT_TIMEOUT, connection.next()) => {
                match result {
                    Err(_) => handle_reconnect(&worker.connector, &mut connection, &worker.store).await,
                    Ok(binance_result) => handle_connection_recv(binance_result, &worker.connector, &mut connection, &worker.store).await,
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

async fn subscribe<T: AsRef<str>>(
    ids: &[T],
    connection: &mut BinanceWebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .subscribe_mini_ticker_stream(&ids.iter().map(|s| s.as_ref()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_subscribe_recv(
    ids: Vec<String>,
    worker_store: &WorkerStore,
    connection: &mut BinanceWebSocketConnection,
) {
    let to_sub = worker_store.add_query_ids(ids).await;
    match subscribe(&to_sub, connection).await {
        Ok(_) => info!("subscribed to ids {:?}", to_sub),
        Err(e) => {
            error!("failed to subscribe to ids {:?}: {}", to_sub, e);
            worker_store.remove_query_ids(to_sub).await;
        }
    }
}

async fn unsubscribe<T: AsRef<str>>(
    ids: &[T],
    connection: &mut BinanceWebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .unsubscribe_mini_ticker_stream(&ids.iter().map(|s| s.as_ref()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_unsubscribe_recv(
    ids: Vec<String>,
    worker_store: &WorkerStore,
    connection: &mut BinanceWebSocketConnection,
) {
    let to_unsub = worker_store.remove_query_ids(ids).await;
    match unsubscribe(&to_unsub, connection).await {
        Ok(_) => info!("unsubscribed to ids {:?}", to_unsub),
        Err(e) => {
            error!("failed to unsubscribe to ids {:?}: {}", to_unsub, e);
            worker_store.add_query_ids(to_unsub).await;
        }
    }
}

async fn handle_reconnect(
    connector: &BinanceWebSocketConnector,
    connection: &mut BinanceWebSocketConnection,
    query_ids: &WorkerStore,
) {
    let mut retry_count: usize = 1;
    loop {
        warn!("reconnecting: attempt {}", retry_count);

        if let Ok(new_connection) = connector.connect().await {
            *connection = new_connection;

            // Resubscribe to all ids
            let ids = query_ids
                .get_query_ids()
                .await
                .into_iter()
                .collect::<Vec<String>>();
            match subscribe(&ids, connection).await {
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

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

async fn store_data(data: Data, store: &WorkerStore) -> Result<(), ParseError> {
    match data {
        Data::MiniTicker(ticker) => {
            let asset_info = AssetInfo {
                id: ticker.symbol.to_lowercase(),
                price: Decimal::from_str_exact(&ticker.close_price)?,
                timestamp: ticker.event_time / 1000,
            };

            store.set_asset(asset_info.id.clone(), asset_info).await;
        }
    }

    Ok(())
}

async fn process_response(resp: BinanceResponse, store: &WorkerStore) {
    match resp {
        BinanceResponse::Stream(resp) => match store_data(resp.data, store).await {
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
    recv_result: Result<BinanceResponse, MessageError>,
    connector: &BinanceWebSocketConnector,
    connection: &mut BinanceWebSocketConnection,
    store: &WorkerStore,
) {
    match recv_result {
        Ok(resp) => {
            process_response(resp, store).await;
        }
        Err(MessageError::ChannelClosed) => {
            handle_reconnect(connector, connection, store).await;
        }
        Err(MessageError::UnsupportedMessage) => {
            error!("unsupported message received from binance");
        }
        Err(MessageError::Parse(..)) => {
            error!("unable to parse message from binance");
        }
    }
}
