use std::sync::Weak;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, trace, warn};

use bothan_core::store::WorkerStore;
use bothan_core::types::AssetInfo;

use crate::api::error::{MessageError, SendError};
use crate::api::types::{HtxResponse, Pong, Tick};
use crate::api::{HtxWebSocketConnection, HtxWebSocketConnector};
use crate::worker::error::WorkerError;
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::HtxWorker;

pub(crate) async fn start_asset_worker(
    worker: Weak<HtxWorker>,
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
async fn handle_reconnect(
    connector: &HtxWebSocketConnector,
    connection: &mut HtxWebSocketConnection,
    store: &WorkerStore,
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

/// Parses a tick response into an AssetInfo structure.
fn parse_tick(id: &str, tick: Tick) -> Result<AssetInfo, WorkerError> {
    let price_value =
        Decimal::from_f64(tick.last_price).ok_or(WorkerError::InvalidPrice(tick.last_price))?;
    Ok(AssetInfo::new(id.to_string(), price_value, 0))
}

/// Stores tick information into the worker store.
async fn store_tick(store: &WorkerStore, id: &str, tick: Tick) -> Result<(), WorkerError> {
    store.set_asset(id, parse_tick(id, tick)?).await?;
    trace!("stored data for id {}", id);
    Ok(())
}

/// Processes the response from the Htx API and handles each type accordingly.
async fn process_response(
    resp: HtxResponse,
    store: &WorkerStore,
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
                match store_tick(store, id, data.tick).await {
                    Ok(_) => debug!("saved data"),
                    Err(e) => error!("failed to save data: {}", e),
                }
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
async fn handle_connection_recv(
    recv_result: Result<HtxResponse, MessageError>,
    connector: &HtxWebSocketConnector,
    connection: &mut HtxWebSocketConnection,
    store: &WorkerStore,
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

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_parse_tick() {
        // Create a mock Tick struct with valid data
        let tick = Tick {
            open: 51732.0,
            high: 52785.64,
            low: 51000.0,
            close: 52735.63,
            amount: 13259.24137056181,
            vol: 687640987.4125315,
            count: 448737,
            bid: 52732.88,
            bid_size: 0.036,
            ask: 52732.89,
            ask_size: 0.583653,
            last_price: 52735.63,
            last_size: 0.03,
        };

        // Parse the tick into AssetInfo
        let result = parse_tick("btcusdt", tick);

        // Expected AssetInfo object
        let expected = AssetInfo::new(
            "btcusdt".to_string(),
            Decimal::from_str("52735.63").unwrap(),
            0,
        );

        // Assert that the parsed result matches the expected output
        assert_eq!(result.as_ref().unwrap().id, expected.id);
        assert_eq!(result.unwrap().price, expected.price);
    }

    #[test]
    fn test_parse_tick_with_failure() {
        // Create a mock Tick struct with an invalid price
        let tick = Tick {
            open: 51732.0,
            high: 52785.64,
            low: 51000.0,
            close: 52735.63,
            amount: 13259.24137056181,
            vol: 687640987.4125315,
            count: 448737,
            bid: 52732.88,
            bid_size: 0.036,
            ask: 52732.89,
            ask_size: 0.583653,
            last_price: f64::INFINITY, // Invalid price to trigger error
            last_size: 0.03,
        };

        // Assert that parsing the tick with an invalid price results in an error
        assert!(parse_tick("btcusdt", tick).is_err());
    }
}
