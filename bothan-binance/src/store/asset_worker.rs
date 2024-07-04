use std::sync::Weak;

use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info};

use bothan_core::store::Store;
use bothan_core::types::AssetInfo;

use crate::api::types::{BinanceResponse, Data};
use crate::api::{self, BinanceWebSocketConnection, BinanceWebSocketConnector};
use crate::store::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::store::BinanceWorker;

pub(crate) async fn start_asset_worker(
    worker: Weak<BinanceWorker>,
    mut connection: BinanceWebSocketConnection,
    mut subscribe_rx: Receiver<Vec<String>>,
    mut unsubscribe_rx: Receiver<Vec<String>>,
) {
    while let Some(worker) = worker.upgrade() {
        select! {
            id = subscribe_rx.recv() => handle_subscribe_recv(id, &mut connection).await,
            id = unsubscribe_rx.recv() => handle_unsubscribe_recv(id, &mut connection).await,
            result = timeout(DEFAULT_TIMEOUT, connection.next()) => {
                match result {
                    Err(_) => handle_reconnect(&worker.connector, &mut connection, &worker.store).await,
                    Ok(binance_result) => handle_connection_recv(binance_result, &worker.connector, &mut connection, &worker.store).await,
                }
            }
        }
    }
}

async fn subscribe(
    ids: Vec<String>,
    connection: &mut BinanceWebSocketConnection,
) -> Result<(), anyhow::Error> {
    if !ids.is_empty() {
        connection
            .subscribe_mini_ticker_stream(&ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_subscribe_recv(
    ids: Option<Vec<String>>,
    connection: &mut BinanceWebSocketConnection,
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
            // Panic here as channel should never close itself
            panic!("subscribe channel closed")
        }
    }
}

async fn handle_unsubscribe_recv(
    ids: Option<Vec<String>>,
    connection: &mut BinanceWebSocketConnection,
) {
    match ids {
        Some(ids) => {
            if ids.is_empty() {
                debug!("received empty unsubscribe command");
            } else {
                let res = connection
                    .unsubscribe_mini_ticker_stream(
                        &ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                    )
                    .await;
                if res.is_err() {
                    error!("failed to unsubscribe to ids: {:?}", ids);
                } else {
                    info!("unsubscribed to ids: {:?}", ids);
                }
            }
        }
        None => {
            // Panic here as channel should never close itself
            panic!("unsubscribe channel closed")
        }
    }
}

async fn handle_reconnect(
    connector: &BinanceWebSocketConnector,
    connection: &mut BinanceWebSocketConnection,
    query_ids: &Store,
) {
    loop {
        if let Ok(new_connection) = connector.connect().await {
            *connection = new_connection;

            // Resubscribe to all ids
            let ids = query_ids.get_query_ids().await;
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

async fn store_data(data: Data, store: &Store) -> anyhow::Result<()> {
    match data {
        Data::MiniTicker(ticker) => {
            let asset_info = AssetInfo {
                id: ticker.symbol.to_lowercase(),
                price: Decimal::from_str_exact(&ticker.close_price)?,
                timestamp: ticker.event_time,
            };

            store.set_asset(asset_info.id.clone(), asset_info).await;
        }
    }

    Ok(())
}

async fn process_response(resp: BinanceResponse, store: &Store) {
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
    recv_result: Result<BinanceResponse, api::Error>,
    connector: &BinanceWebSocketConnector,
    connection: &mut BinanceWebSocketConnection,
    store: &Store,
) {
    match recv_result {
        Ok(resp) => {
            process_response(resp, store).await;
        }
        Err(api::Error::ChannelClosed) => {
            handle_reconnect(connector, connection, store).await;
        }
        Err(api::Error::UnsupportedMessage) => {
            error!("unsupported message received from binance");
        }
        Err(api::Error::Parse(..)) => {
            error!("unable to parse message from binance");
        }
    }
}
