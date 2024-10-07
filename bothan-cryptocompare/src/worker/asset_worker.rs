use std::sync::Weak;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tokio_tungstenite::tungstenite;
use tracing::{debug, error, info, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::errors::MessageError;
use crate::api::msgs::{Packet, ReferenceTick};
use crate::api::{CryptoCompareWebSocketConnection, CryptoCompareWebSocketConnector};
use crate::worker::errors::WorkerError;
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::CryptoCompareWorker;

pub(crate) async fn start_asset_worker(
    weak_worker: Weak<CryptoCompareWorker>,
    mut connection: CryptoCompareWebSocketConnection,
    mut subscribe_rx: Receiver<Vec<String>>,
    mut unsubscribe_rx: Receiver<Vec<String>>,
) {
    while let Some(worker) = weak_worker.upgrade() {
        select! {
            Some(ids) = subscribe_rx.recv() => handle_subscribe_recv(ids, &worker.store, &mut connection).await,
            Some(ids) = unsubscribe_rx.recv() => handle_unsubscribe_recv(ids, &worker.store, &mut connection).await,
            result = timeout(DEFAULT_TIMEOUT, connection.next()) => {
                match result {
                    Err(_) => handle_reconnect(&worker.connector, &mut connection, &worker.store).await,
                    Ok(packet_result) => handle_connection_recv(packet_result, &worker.connector, &mut connection, &worker.store).await,
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
    connection: &mut CryptoCompareWebSocketConnection,
) -> Result<(), tungstenite::Error> {
    if !ids.is_empty() {
        connection
            .subscribe_latest_tick_adaptive_inclusion(
                &ids.iter().map(|s| s.as_ref()).collect::<Vec<&str>>(),
            )
            .await?
    }

    Ok(())
}

async fn handle_subscribe_recv(
    ids: Vec<String>,
    worker_store: &WorkerStore,
    connection: &mut CryptoCompareWebSocketConnection,
) {
    match subscribe(&ids, connection).await {
        Ok(_) => info!("subscribed to ids {:?}", ids),
        Err(e) => {
            error!("failed to subscribe to ids {:?}: {}", ids, e);
            if worker_store.remove_query_ids(ids).await.is_err() {
                error!("failed to remove query ids from store")
            }
        }
    }
}

async fn unsubscribe<T: AsRef<str>>(
    ids: &[T],
    connection: &mut CryptoCompareWebSocketConnection,
) -> Result<(), tungstenite::Error> {
    if !ids.is_empty() {
        connection
            .unsubscribe_latest_tick_adaptive_inclusion(
                &ids.iter().map(|s| s.as_ref()).collect::<Vec<&str>>(),
            )
            .await?
    }

    Ok(())
}

async fn handle_unsubscribe_recv(
    ids: Vec<String>,
    worker_store: &WorkerStore,
    connection: &mut CryptoCompareWebSocketConnection,
) {
    match unsubscribe(&ids, connection).await {
        Ok(_) => info!("unsubscribed to ids {:?}", ids),
        Err(e) => {
            error!("failed to unsubscribe to ids {:?}: {}", ids, e);
            if worker_store.add_query_ids(ids).await.is_err() {
                error!("failed to add query ids to store")
            }
        }
    }
}

async fn handle_reconnect(
    connector: &CryptoCompareWebSocketConnector,
    connection: &mut CryptoCompareWebSocketConnection,
    query_ids: &WorkerStore,
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

            if subscribe(&ids_vec, connection).await.is_ok() {
                info!("resubscribed to all ids");
            } else {
                error!("failed to resubscribe to all ids");
            }
        } else {
            error!("failed to reconnect to binance");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

async fn store_ref_tick(store: &WorkerStore, ref_tick: ReferenceTick) -> Result<(), WorkerError> {
    let id = ref_tick.instrument.clone();
    let price =
        Decimal::from_f64(ref_tick.value).ok_or(WorkerError::InvalidDecimal(ref_tick.value))?;

    let asset_info = AssetInfo::new(ref_tick.instrument, price, ref_tick.last_update);
    store.set_asset(id, asset_info).await?;

    Ok(())
}

async fn process_packet(store: &WorkerStore, packet: Packet) {
    match packet {
        Packet::RefTickAdaptive(ref_tick) => match store_ref_tick(store, ref_tick).await {
            Ok(_) => info!("stored data"),
            Err(e) => error!("failed to store ref tick: {}", e),
        },
        Packet::SubscriptionError(msg) => error!("subscription error: {}", msg),
        Packet::SubscriptionRejected(msg) => error!("subscription rejected: {}", msg),
        Packet::SubscriptionWarning(msg) => warn!("subscription warning: {}", msg),
        _ => (),
    }
}

async fn handle_connection_recv(
    result: Result<Packet, MessageError>,
    connector: &CryptoCompareWebSocketConnector,
    connection: &mut CryptoCompareWebSocketConnection,
    store: &WorkerStore,
) {
    match result {
        Ok(resp) => process_packet(store, resp).await,
        Err(MessageError::ChannelClosed) => handle_reconnect(connector, connection, store).await,
        Err(MessageError::UnsupportedMessage) => {
            error!("unsupported message received from cryptocompare")
        }
        Err(MessageError::Parse(_)) => error!("unable to parse message from cryptocompare"),
    }
}
