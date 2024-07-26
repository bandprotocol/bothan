use std::sync::Weak;

use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::store::Store;
use bothan_core::types::AssetInfo;

use crate::api::error::{MessageError, SendError};
use crate::api::types::channels::Channel;
use crate::api::types::{CoinbaseResponse, Ticker};
use crate::api::{CoinbaseWebSocketConnection, CoinbaseWebSocketConnector};
use crate::worker::error::ParseError;
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::CoinbaseWorker;

pub(crate) async fn start_asset_worker(
    worker: Weak<CoinbaseWorker>,
    mut connection: CoinbaseWebSocketConnection,
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

async fn subscribe(
    ids: &[String],
    connection: &mut CoinbaseWebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .subscribe(
                vec![Channel::Ticker],
                &ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            )
            .await?
    }

    Ok(())
}

async fn handle_subscribe_recv(ids: Vec<String>, connection: &mut CoinbaseWebSocketConnection) {
    if let Err(e) = subscribe(&ids, connection).await {
        error!("failed to subscribe to ids {:?}: {}", ids, e);
    } else {
        info!("subscribed to ids {:?}", ids);
    }
}

async fn unsubscribe(
    ids: &[String],
    connection: &mut CoinbaseWebSocketConnection,
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

async fn handle_unsubscribe_recv(ids: Vec<String>, connection: &mut CoinbaseWebSocketConnection) {
    if let Err(e) = unsubscribe(&ids, connection).await {
        error!("failed to unsubscribe to ids {:?}: {}", ids, e);
    } else {
        info!("subscribed to ids {:?}", ids);
    }
}

async fn handle_reconnect(
    connector: &CoinbaseWebSocketConnector,
    connection: &mut CoinbaseWebSocketConnection,
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
            error!("failed to reconnect to coinbase");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

fn parse_ticker(ticker: &Ticker) -> Result<AssetInfo, ParseError> {
    let price_value = Decimal::from_str_exact(&ticker.price)?;
    Ok(AssetInfo::new(
        ticker.product_id.clone(),
        price_value,
        chrono::Utc::now().timestamp(),
    ))
}

async fn store_tickers(ticker: &Ticker, store: &Store) -> Result<(), ParseError> {
    let id = ticker.product_id.clone();
    match parse_ticker(ticker) {
        Ok(asset_info) => store.set_asset(id.clone(), asset_info).await,
        Err(e) => {
            warn!("failed to parse ticker data for {} with error {}", id, e);
            return Err(e);
        }
    }

    Ok(())
}

/// Processes the response from the Coinbase API.
async fn process_response(resp: CoinbaseResponse, store: &Store) {
    match resp {
        CoinbaseResponse::Ticker(ticker) => match store_tickers(&ticker, store).await {
            Ok(_) => info!("saved data"),
            Err(e) => error!("failed to save data: {}", e),
        },
        CoinbaseResponse::Subscriptions(_) => {
            info!("received request response");
        }
    }
}

async fn handle_connection_recv(
    recv_result: Result<CoinbaseResponse, MessageError>,
    connector: &CoinbaseWebSocketConnector,
    connection: &mut CoinbaseWebSocketConnection,
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
