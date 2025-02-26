use std::sync::Weak;

use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::types::AssetInfo;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use crate::api::error::{MessageError, SendError};
use crate::api::types::CoinbaseResponse;
use crate::api::types::channels::Channel;
use crate::api::{Ticker, WebSocketConnection, WebSocketConnector};
use crate::worker::InnerWorker;
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};

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
                        Ok(coinbase_result) => handle_connection_recv(coinbase_result, &worker.connector, &mut connection, &worker.store).await,
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
        connection
            .subscribe(vec![Channel::Ticker], &ids_vec)
            .await?
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
            .unsubscribe(
                vec![Channel::Ticker],
                &ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            )
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
            error!("failed to reconnect to coinbase");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

fn parse_ticker(ticker: &Ticker) -> Result<AssetInfo, rust_decimal::Error> {
    let id = ticker.product_id.clone();
    let price_value = Decimal::from_str_exact(&ticker.price)?;
    let timestamp = chrono::Utc::now().timestamp();
    Ok(AssetInfo::new(id, price_value, timestamp))
}

async fn store_ticker<S: Store>(store: &WorkerStore<S>, ticker: &Ticker) {
    let id = ticker.product_id.clone();
    let asset_info = match parse_ticker(ticker) {
        Ok(ticker) => ticker,
        Err(e) => {
            error!("failed to parse ticker: {}", e);
            return;
        }
    };
    match store.set_asset_info(asset_info).await {
        Ok(_) => debug!("stored data for id {}", id),
        Err(e) => error!("failed to save data: {}", e),
    }
}

/// Processes the response from the Coinbase API.
async fn process_response<S: Store>(resp: CoinbaseResponse, store: &WorkerStore<S>) {
    match resp {
        CoinbaseResponse::Ticker(ticker) => store_ticker(store, &ticker).await,
        CoinbaseResponse::Ping => debug!("received ping"),
        CoinbaseResponse::Subscriptions(_) => {
            info!("received request response");
        }
    }
}

async fn handle_connection_recv<S: Store>(
    recv_result: Result<CoinbaseResponse, MessageError>,
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
            error!("unsupported message received from coinbase");
        }
        Err(MessageError::Parse(..)) => {
            error!("unable to parse message from coinbase");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_market() {
        let ticker = Ticker {
            sequence: 1,
            product_id: "BTC-USD".to_string(),
            price: "42000.99".to_string(),
            open_24h: "9000.00".to_string(),
            volume_24h: "1000.00".to_string(),
            low_24h: "9500.00".to_string(),
            high_24h: "10500.00".to_string(),
            volume_30d: "30000.00".to_string(),
            best_bid: "9999.00".to_string(),
            best_bid_size: "0.01".to_string(),
            best_ask: "10001.00".to_string(),
            best_ask_size: "0.01".to_string(),
            side: "buy".to_string(),
            time: "2021-01-01T00:00:00.000Z".to_string(),
            trade_id: 1,
            last_size: "0.01".to_string(),
        };
        let result = parse_ticker(&ticker);
        let expected = AssetInfo::new(
            "BTC-USD".to_string(),
            Decimal::from_str_exact("42000.99").unwrap(),
            0,
        );
        assert_eq!(result.as_ref().unwrap().id, expected.id);
        assert_eq!(result.unwrap().price, expected.price);
    }

    #[test]
    fn test_parse_market_with_failure() {
        let ticker = Ticker {
            sequence: 1,
            product_id: "BTC-USD".to_string(),
            price: f64::INFINITY.to_string(),
            open_24h: "9000.00".to_string(),
            volume_24h: "1000.00".to_string(),
            low_24h: "9500.00".to_string(),
            high_24h: "10500.00".to_string(),
            volume_30d: "30000.00".to_string(),
            best_bid: "9999.00".to_string(),
            best_bid_size: "0.01".to_string(),
            best_ask: "10001.00".to_string(),
            best_ask_size: "0.01".to_string(),
            side: "buy".to_string(),
            time: "2021-01-01T00:00:00.000Z".to_string(),
            trade_id: 1,
            last_size: "0.01".to_string(),
        };
        assert!(parse_ticker(&ticker).is_err());
    }
}
