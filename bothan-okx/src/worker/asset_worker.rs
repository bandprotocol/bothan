use std::sync::Weak;

use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::error::{MessageError, SendError};
use crate::api::types::{ChannelResponse, OkxResponse, TickerData};
use crate::api::{OkxWebSocketConnection, OkxWebSocketConnector};
use crate::worker::error::WorkerError;
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
        let ids_vec = ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        connection.subscribe_ticker(&ids_vec).await?
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
        info!("unsubscribed to ids {:?}", ids);
    }
}

async fn handle_reconnect(
    connector: &OkxWebSocketConnector,
    connection: &mut OkxWebSocketConnection,
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

fn parse_ticker(ticker: TickerData) -> Result<AssetInfo, WorkerError> {
    let id = ticker.inst_id.clone();
    let price_value = Decimal::from_str_exact(&ticker.last)?;
    Ok(AssetInfo::new(
        id,
        price_value,
        chrono::Utc::now().timestamp(),
    ))
}

async fn store_ticker(store: &WorkerStore, ticker: TickerData) -> Result<(), WorkerError> {
    store
        .set_asset(ticker.inst_id.clone(), parse_ticker(ticker)?)
        .await?;
    Ok(())
}

/// Processes the response from the Okx API.
async fn process_response(resp: OkxResponse, store: &WorkerStore) {
    match resp {
        OkxResponse::ChannelResponse(resp) => match resp {
            ChannelResponse::Ticker(push_data) => {
                let tickers = push_data.data;
                for ticker in tickers {
                    match store_ticker(store, ticker).await {
                        Ok(_) => info!("saved data"),
                        Err(e) => error!("failed to save data: {}", e),
                    }
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
            error!("unsupported message received from okx");
        }
        Err(MessageError::Parse(..)) => {
            error!("unable to parse message from okx");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_market() {
        let ticker = TickerData {
            inst_type: "SPOT".to_string(),
            inst_id: "BTC-USDT".to_string(),
            last: "42000.99".to_string(),
            last_sz: "5000".to_string(),
            ask_px: "10001".to_string(),
            ask_sz: "5000".to_string(),
            bid_px: "9999".to_string(),
            bid_sz: "5000".to_string(),
            open_24h: "10000".to_string(),
            high_24h: "10000".to_string(),
            low_24h: "10000".to_string(),
            vol_ccy_24h: "10000".to_string(),
            vol_24h: "10000".to_string(),
            sod_utc0: "10000".to_string(),
            sod_utc8: "10000".to_string(),
            ts: "10000".to_string(),
        };
        let result = parse_ticker(ticker);
        let expected = AssetInfo::new(
            "BTC-USDT".to_string(),
            Decimal::from_str_exact("42000.99").unwrap(),
            0,
        );
        assert_eq!(result.as_ref().unwrap().id, expected.id);
        assert_eq!(result.unwrap().price, expected.price);
    }

    #[test]
    fn test_parse_market_with_failure() {
        let ticker = TickerData {
            inst_type: "SPOT".to_string(),
            inst_id: "BTC-USDT".to_string(),
            last: f64::INFINITY.to_string(),
            last_sz: "5000".to_string(),
            ask_px: "10001".to_string(),
            ask_sz: "5000".to_string(),
            bid_px: "9999".to_string(),
            bid_sz: "5000".to_string(),
            open_24h: "10000".to_string(),
            high_24h: "10000".to_string(),
            low_24h: "10000".to_string(),
            vol_ccy_24h: "10000".to_string(),
            vol_24h: "10000".to_string(),
            sod_utc0: "10000".to_string(),
            sod_utc8: "10000".to_string(),
            ts: "10000".to_string(),
        };
        assert!(parse_ticker(ticker).is_err());
    }
}
