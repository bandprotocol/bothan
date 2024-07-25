use std::sync::Weak;

use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::store::Store;
use bothan_core::types::AssetInfo;

use crate::api::error::{MessageError, SendError};
use crate::api::types::{ChannelResponse, OkxResponse, TickerData};
use crate::api::{OkxWebSocketConnection, OkxWebSocketConnector};
use crate::worker::error::ParseError;
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::OkxWorker;

pub(crate) async fn start_asset_worker(
    worker: Weak<OkxWorker>,
    mut connection: OkxWebSocketConnection,
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

async fn subscribe(
    ids: &[String],
    connection: &mut OkxWebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .subscribe_ticker(&ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_subscribe_recv(ids: Vec<String>, connection: &mut OkxWebSocketConnection) {
    if let Err(e) = subscribe(&ids, connection).await {
        error!("failed to subscribe to ids {:?}: {}", ids, e);
    } else {
        info!("subscribed to ids {:?}", ids);
    }
}

async fn unsubscribe(
    ids: &[String],
    connection: &mut OkxWebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .unsubscribe_ticker(&ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_unsubscribe_recv(ids: Vec<String>, connection: &mut OkxWebSocketConnection) {
    if let Err(e) = unsubscribe(&ids, connection).await {
        error!("failed to unsubscribe to ids {:?}: {}", ids, e);
    } else {
        info!("subscribed to ids {:?}", ids);
    }
}

async fn handle_reconnect(
    connector: &OkxWebSocketConnector,
    connection: &mut OkxWebSocketConnection,
    query_ids: &Store,
) {
    let mut retry_count: usize = 1;
    loop {
        warn!("reconnecting: attempt {}", retry_count);

        if let Ok(new_connection) = connector.connect().await {
            *connection = new_connection;

            // Resubscribe to all ids
            let ids = query_ids.get_query_ids().await;
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
            error!("failed to reconnect to okx");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

fn parse_ticker(ticker: TickerData) -> Result<AssetInfo, ParseError> {
    let price_value = Decimal::from_str_exact(&ticker.last)?;
    Ok(AssetInfo::new(
        ticker.inst_id,
        price_value,
        chrono::Utc::now().timestamp(),
    ))
}

async fn store_tickers(tickers: Vec<TickerData>, store: &Store) -> Result<(), ParseError> {
    let to_set = tickers
        .into_iter()
        .filter_map(|ticker| {
            let id = ticker.inst_id.clone();
            match parse_ticker(ticker) {
                Ok(asset_info) => Some((id, asset_info)),
                Err(e) => {
                    warn!("failed to parse ticker data for {} with error {}", id, e);
                    None
                }
            }
        })
        .collect::<Vec<(String, AssetInfo)>>();

    store.set_assets(to_set).await;
    Ok(())
}

/// Processes the response from the Okx API.
async fn process_response(resp: OkxResponse, store: &Store) {
    match resp {
        OkxResponse::ChannelResponse(resp) => match resp {
            ChannelResponse::Ticker(push_data) => {
                match store_tickers(push_data.data, store).await {
                    Ok(_) => info!("saved data"),
                    Err(e) => error!("failed to save data: {}", e),
                }
            }
        },
        OkxResponse::WebSocketMessageResponse(resp) => {
            debug!("received public message from okx: {:?}", resp);
        }
    }
}

async fn handle_connection_recv(
    recv_result: Result<OkxResponse, MessageError>,
    connector: &OkxWebSocketConnector,
    connection: &mut OkxWebSocketConnection,
    store: &Store,
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
