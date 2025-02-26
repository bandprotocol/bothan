use std::collections::HashMap;
use std::sync::Weak;

use opentelemetry::{global, KeyValue};
use rand::random;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::types::AssetInfo;

use crate::api::error::MessageError;
use crate::api::msgs::{BinanceResponse, Data, ErrorResponse, SuccessResponse};
use crate::api::{WebSocketConnection, WebSocketConnector};
use crate::worker::types::{DEFAULT_TIMEOUT, METER_NAME, RECONNECT_BUFFER};
use crate::worker::InnerWorker;

enum Event {
    Subscribe(Vec<String>),
    Unsubscribe(Vec<String>),
}

pub(crate) async fn start_asset_worker<S: Store>(
    inner_worker: Weak<InnerWorker<S>>,
    mut connection: WebSocketConnection,
    mut subscribe_rx: Receiver<Vec<String>>,
    mut unsubscribe_rx: Receiver<Vec<String>>,
) {
    let mut subscription_map = HashMap::new();
    while let Some(worker) = inner_worker.upgrade() {
        select! {
            Some(ids) = subscribe_rx.recv() => handle_subscribe_recv(ids, &mut connection, &mut subscription_map).await,
            Some(ids) = unsubscribe_rx.recv() => handle_unsubscribe_recv(ids, &mut connection, &mut subscription_map).await,
            result = timeout(DEFAULT_TIMEOUT, connection.next()) => {
                match result {
                    // Assume that the connection has been closed on timeout and attempt to reconnect
                    Err(_) => handle_reconnect(&worker.connector, &mut connection, &worker.store).await,
                    Ok(binance_result) => handle_connection_recv(binance_result, &worker.connector, &mut connection, &worker.store, &mut subscription_map).await,
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

async fn handle_subscribe_recv(
    ids: Vec<String>,
    connection: &mut WebSocketConnection,
    subscription_map: &mut HashMap<i64, Event>,
) {
    if ids.is_empty() {
        return;
    }

    let packet_id = random();
    let tickers = ids.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();

    let meter = global::meter(METER_NAME);
    meter.u64_counter("subscribe_attempt").build().add(
        1,
        &[
            KeyValue::new("subscription.id", packet_id),
            KeyValue::new("subscription.tickers", tickers.join(",")),
        ],
    );

    match connection
        .subscribe_mini_ticker_stream(packet_id, &tickers)
        .await
    {
        Ok(_) => {
            info!("attempt to subscribe to ids {:?}", ids);
            subscription_map.insert(packet_id, Event::Subscribe(ids));
        }
        Err(e) => {
            error!("failed attempt to subscribe to ids {:?}: {}", ids, e);
            meter
                .u64_counter("failed_subscribe_attempt")
                .build()
                .add(1, &[KeyValue::new("subscription.id", packet_id)]);
        }
    }
}

async fn handle_unsubscribe_recv(
    ids: Vec<String>,
    connection: &mut WebSocketConnection,
    subscription_map: &mut HashMap<i64, Event>,
) {
    if ids.is_empty() {
        return;
    }

    let packet_id = random();
    let tickers = ids.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();

    let meter = global::meter(METER_NAME);
    meter.u64_counter("unsubscribe_attempt").build().add(
        1,
        &[
            KeyValue::new("subscription.id", packet_id),
            KeyValue::new("subscription.tickers", tickers.join(",")),
        ],
    );

    match connection
        .unsubscribe_mini_ticker_stream(packet_id, &tickers)
        .await
    {
        Ok(_) => {
            info!("attempt to unsubscribe to ids {:?}", ids);
            subscription_map.insert(packet_id, Event::Unsubscribe(ids));
        }
        Err(e) => {
            error!("failed attempt to unsubscribe to ids {:?}: {}", ids, e);
            meter
                .u64_counter("failed_unsubscribe_attempt")
                .build()
                .add(1, &[KeyValue::new("subscription.id", packet_id)]);
        }
    }
}

async fn handle_connection_recv<S: Store>(
    recv_result: Result<BinanceResponse, MessageError>,
    connector: &WebSocketConnector,
    connection: &mut WebSocketConnection,
    store: &WorkerStore<S>,
    subscription_map: &mut HashMap<i64, Event>,
) {
    match recv_result {
        Ok(resp) => {
            process_response(store, resp, subscription_map).await;
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

async fn handle_reconnect<S: Store>(
    connector: &WebSocketConnector,
    connection: &mut WebSocketConnection,
    query_ids: &WorkerStore<S>,
) {
    let mut retry_count: usize = 1;
    loop {
        let meter = global::meter(METER_NAME);
        meter.u64_counter("reconnect-attempts").build().add(1, &[]);

        warn!("reconnecting: attempt {}", retry_count);

        if let Ok(new_connection) = connector.connect().await {
            *connection = new_connection;

            // Resubscribe to all ids
            let Ok(ids) = query_ids.get_query_ids().await else {
                error!("failed to get query ids from store");
                return;
            };

            let ids_vec = ids.into_iter().collect::<Vec<String>>();

            if ids_vec.is_empty() {
                info!("no ids to resubscribe to");
                return;
            }

            let packet_id = random();

            if connection
                .subscribe_mini_ticker_stream(packet_id, &ids_vec)
                .await
                .is_ok()
            {
                info!("resubscribed to all ids");
                return;
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

async fn process_response<S: Store>(
    store: &WorkerStore<S>,
    resp: BinanceResponse,
    subscription_map: &mut HashMap<i64, Event>,
) {
    match resp {
        BinanceResponse::Stream(r) => store_data(store, r.data).await,
        BinanceResponse::Success(r) => process_success(store, r, subscription_map).await,
        BinanceResponse::Ping => process_ping(),
        BinanceResponse::Error(e) => process_error(e),
    }
}

async fn store_data<S: Store>(store: &WorkerStore<S>, data: Data) {
    match data {
        Data::MiniTicker(ticker) => {
            let id = ticker.symbol.to_lowercase();
            let Ok(price) = Decimal::from_str_exact(&ticker.close_price) else {
                error!("failed to parse price for id {}", id);
                return;
            };

            let timestamp = ticker.event_time / 1000;
            let asset_info = AssetInfo::new(id.clone(), price, timestamp);

            match store.set_asset_info(asset_info).await {
                Ok(_) => {
                    info!("stored data for id {}", id);
                    global::meter(METER_NAME)
                        .f64_gauge("asset-prices")
                        .build()
                        .record(
                            price.to_f64().unwrap(), // Prices should never be NaN so unwrap here
                            &[KeyValue::new("asset.symbol", id)],
                        );
                }
                Err(e) => error!("failed to store data for id {}: {}", id, e),
            }
        }
    }
}

async fn process_success<S: Store>(
    store: &WorkerStore<S>,
    success_response: SuccessResponse,
    subscription_map: &mut HashMap<i64, Event>,
) {
    let meter = global::meter(METER_NAME);

    match subscription_map.remove(&success_response.id) {
        Some(Event::Subscribe(ids)) => {
            info!("subscribed to ids {:?}", ids);
            meter
                .u64_counter("subscribe_success")
                .build()
                .add(1, &[KeyValue::new("id", success_response.id)]);
            if store.add_query_ids(ids).await.is_err() {
                error!("failed to add query ids to store");
            };
        }
        Some(Event::Unsubscribe(ids)) => {
            info!("unsubscribed to ids {:?}", ids);
            meter
                .u64_counter("unsubscribe_success")
                .build()
                .add(1, &[KeyValue::new("subscription.id", success_response.id)]);
            if store.remove_query_ids(&ids).await.is_err() {
                error!("failed to remove query ids from store");
            };
        }
        None => error!("received response for unknown id: {}", success_response.id),
    }
}

fn process_ping() {
    debug!("received ping from binance");
    global::meter(METER_NAME)
        .u64_counter("pings")
        .build()
        .add(1, &[]);
}

fn process_error(error: ErrorResponse) {
    error!(
        "error code {} received from binance: {}",
        error.code, error.msg
    );
    global::meter(METER_NAME).u64_counter("errors").build().add(
        1,
        &[
            KeyValue::new("msg.code", error.code as i64),
            KeyValue::new("msg.error", error.msg),
        ],
    );
}
