use std::sync::Weak;

use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::store::Store;
use bothan_core::types::AssetInfo;

use crate::api::error::{MessageError, SendError};
use crate::api::types::{ChannelResponse, KrakenResponse, TickerResponse};
use crate::api::{KrakenWebSocketConnection, KrakenWebSocketConnector};
use crate::worker::error::ParseError;
use crate::worker::types::{DEFAULT_TIMEOUT, RECONNECT_BUFFER};
use crate::worker::KrakenWorker;

pub(crate) async fn start_asset_worker(
    worker: Weak<KrakenWorker>,
    mut connection: KrakenWebSocketConnection,
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
                        Ok(kraken_result) => handle_connection_recv(kraken_result, &worker.connector, &mut connection, &worker.store).await,
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
    connection: &mut KrakenWebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .subscribe_ticker(
                &ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                None,
                None,
            )
            .await?
    }

    Ok(())
}

async fn handle_subscribe_recv(ids: Vec<String>, connection: &mut KrakenWebSocketConnection) {
    if let Err(e) = subscribe(&ids, connection).await {
        error!("failed to subscribe to ids {:?}: {}", ids, e);
    } else {
        info!("subscribed to ids {:?}", ids);
    }
}

async fn unsubscribe(
    ids: &[String],
    connection: &mut KrakenWebSocketConnection,
) -> Result<(), SendError> {
    if !ids.is_empty() {
        connection
            .unsubscribe_ticker(&ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
            .await?
    }

    Ok(())
}

async fn handle_unsubscribe_recv(ids: Vec<String>, connection: &mut KrakenWebSocketConnection) {
    if let Err(e) = unsubscribe(&ids, connection).await {
        error!("failed to unsubscribe to ids {:?}: {}", ids, e);
    } else {
        info!("subscribed to ids {:?}", ids);
    }
}

async fn handle_reconnect(
    connector: &KrakenWebSocketConnector,
    connection: &mut KrakenWebSocketConnection,
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
            error!("failed to reconnect to kraken");
        }

        retry_count += 1;
        sleep(RECONNECT_BUFFER).await;
    }
}

fn parse_ticker(ticker: TickerResponse) -> Result<AssetInfo, ParseError> {
    let price_value =
        Decimal::from_f64(ticker.last).ok_or(ParseError::InvalidPrice(ticker.last))?;
    Ok(AssetInfo::new(
        ticker.symbol,
        price_value,
        chrono::Utc::now().timestamp(),
    ))
}

async fn store_tickers(tickers: Vec<TickerResponse>, store: &Store) -> Result<(), ParseError> {
    let to_set = tickers
        .into_iter()
        .filter_map(|ticker| {
            let id = ticker.symbol.clone();
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

/// Processes the response from the Kraken API.
async fn process_response(resp: KrakenResponse, store: &Store) {
    match resp {
        KrakenResponse::Channel(resp) => match resp {
            ChannelResponse::Ticker(tickers) => match store_tickers(tickers, store).await {
                Ok(_) => info!("saved data"),
                Err(e) => error!("failed to save data: {}", e),
            },
            ChannelResponse::Heartbeat => {
                debug!("received heartbeat from kraken");
            }
            ChannelResponse::Status(status) => {
                debug!("received status from kraken: {:?}", status);
            }
        },
        KrakenResponse::PublicMessage(resp) => {
            debug!("received public message from kraken: {:?}", resp);
        }
        KrakenResponse::Pong => {
            debug!("received pong from kraken");
        }
    }
}

async fn handle_connection_recv(
    recv_result: Result<KrakenResponse, MessageError>,
    connector: &KrakenWebSocketConnector,
    connection: &mut KrakenWebSocketConnection,
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
            error!("unsupported message received from kraken");
        }
        Err(MessageError::Parse(..)) => {
            error!("unable to parse message from kraken");
        }
    }
}

#[cfg(test)]
mod test {
    use std::f64::INFINITY;

    use super::*;

    #[test]
    fn test_parse_market() {
        let ticker = TickerResponse {
            symbol: "BTC".to_string(),
            bid: 42000.00,
            bid_qty: 50000.00,
            ask: 42001.00,
            ask_qty: 50000.00,
            last: 42000.99,
            volume: 100000.00,
            vwap: 42000.00,
            low: 40000.00,
            high: 44000.00,
            change: 2000.00,
            change_pct: 0.05,
        };
        let result = parse_ticker(ticker);
        let expected = AssetInfo::new(
            "BTC".to_string(),
            Decimal::from_str_exact("42000.99").unwrap(),
            0,
        );
        assert_eq!(result.as_ref().unwrap().id, expected.id);
        assert_eq!(result.unwrap().price, expected.price);
    }

    #[test]
    fn test_parse_market_with_failure() {
        let ticker = TickerResponse {
            symbol: "BTC".to_string(),
            bid: 42000.00,
            bid_qty: 50000.00,
            ask: 42001.00,
            ask_qty: 50000.00,
            last: INFINITY,
            volume: 100000.00,
            vwap: 42000.00,
            low: 40000.00,
            high: 44000.00,
            change: 2000.00,
            change_pct: 0.05,
        };
        assert!(parse_ticker(ticker).is_err());
    }
}
