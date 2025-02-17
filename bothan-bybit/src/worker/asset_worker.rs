use std::sync::Weak;

use crate::api::error::{MessageError, SendError};
use crate::api::types::{BybitResponse, Ticker};
use crate::api::{WebSocketConnection, WebSocketConnector};
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::InnerWorker;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::types::AssetInfo;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

pub(crate) async fn start_asset_worker<S: Store>(
    inner_worker: Weak<InnerWorker<S>>,
    mut connection: WebSocketConnection,
    mut subscribe_rx: Receiver<Vec<String>>,
    mut unsubscribe_rx: Receiver<Vec<String>>,
) {
    loop {
        select! {
            Some(ids) = subscribe_rx.recv() => handle_subscribe_recv(ids, &mut connection).await,
            Some(ids) = unsubscribe_rx.recv() => handle_unsubscribe_recv(ids, &mut connection).await,
            result = timeout(DEFAULT_TIMEOUT, connection.next()) => {
                if let Some(worker) = inner_worker.upgrade() {
                    match result {
                        Err(_) => handle_reconnect(&worker.connector, &mut connection, &worker.store).await,
                        Ok(bybit_result) => handle_connection_recv(bybit_result, &worker.connector, &mut connection, &worker.store).await,
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
        for batched_ids in ids.chunks(MAX_ARGS as usize) {
            let symbols = batched_ids
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();
            connection.subscribe_ticker(&symbols).await?
        }
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
        for batched_ids in ids.chunks(MAX_ARGS as usize) {
            let symbols = batched_ids
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();
            connection.unsubscribe_ticker(&symbols).await?
        }
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
    connector: &WebSocketConnector,
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
            error!("failed to reconnect to bybit");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

fn parse_ticker(ticker: Ticker) -> Result<AssetInfo, rust_decimal::Error> {
    let id = ticker.symbol.clone();
    let price_value = Decimal::from_str_exact(&ticker.last_price)?;
    let timestamp = chrono::Utc::now().timestamp();
    Ok(AssetInfo::new(id, price_value, timestamp))
}

async fn store_ticker<S: Store>(store: &WorkerStore<S>, ticker: Ticker) {
    let id = ticker.symbol.clone();
    let asset_info = match parse_ticker(ticker) {
        Ok(asset_info) => asset_info,
        Err(e) => {
            error!("failed to parse ticker data for id {}: {}", id, e);
            return;
        }
    };

    if let Err(e) = store.set_asset(id.clone(), asset_info).await {
        error!("failed to store ticker data for id {}: {}", id, e);
    } else {
        debug!("stored data for id {}", id);
    }
}

/// Processes the response from the Bybit API.
async fn process_response<S: Store>(resp: BybitResponse, store: &WorkerStore<S>) {
    match resp {
        BybitResponse::PublicTicker(resp) => store_ticker(store, resp.data).await,
        BybitResponse::PublicMessage(resp) => {
            debug!("received public message from Bybit: {:?}", resp)
        }
    }
}

async fn handle_connection_recv<S: Store>(
    recv_result: Result<BybitResponse, MessageError>,
    connector: &WebSocketConnector,
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
            error!("unsupported message received from bybit");
        }
        Err(MessageError::Parse(..)) => {
            error!("unable to parse message from bybit");
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_parse_ticker() {
        // Create a mock ticker response.
        let ticker = Ticker {
            symbol: "BTCUSDT".to_string(),
            last_price: "42000.99".to_string(),
            high_price24h: "44000.00".to_string(),
            low_price24h: "40000.00".to_string(),
            prev_price24h: "40000.00".to_string(),
            volume24h: "100000.00".to_string(),
            turnover24h: "4200000000.00".to_string(),
            price24h_pcnt: "0.05".to_string(),
            usd_index_price: "42000.00".to_string(),
        };

        // Parse the ticker using your custom parse function.
        let result = parse_ticker(ticker);
        let expected = AssetInfo::new(
            "BTCUSDT".to_string(),
            Decimal::from_str("42000.99").unwrap(),
            0,
        );

        // Assertions to check the parsed output matches the expected values.
        assert_eq!(result.as_ref().unwrap().id, expected.id);
        assert_eq!(result.unwrap().price, expected.price);
    }

    #[test]
    fn test_parse_ticker_with_failure() {
        // Create a mock ticker response with an invalid price to simulate failure.
        let ticker = Ticker {
            symbol: "BTCUSDT".to_string(),
            last_price: "NaN".to_string(), // Using NaN to simulate a parsing failure scenario.
            high_price24h: "44000.00".to_string(),
            low_price24h: "40000.00".to_string(),
            prev_price24h: "40000.00".to_string(),
            volume24h: "100000.00".to_string(),
            turnover24h: "4200000000.00".to_string(),
            price24h_pcnt: "0.05".to_string(),
            usd_index_price: "42000.00".to_string(),
        };

        // Expect the parse to fail due to invalid data.
        assert!(parse_ticker(ticker).is_err());
    }
}
