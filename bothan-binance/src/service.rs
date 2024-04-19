use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::error::Error as BinanceError;
use crate::api::types::{BinanceResponse, Data};
use crate::api::websocket::{BinanceWebSocketConnection, BinanceWebSocketConnector};
use crate::types::{Command, DEFAULT_TIMEOUT};

pub struct BinanceService {
    cache: Arc<Cache<PriceData>>,
    cmd_tx: Arc<Sender<Command>>,
}

impl BinanceService {
    pub fn new(
        connector: Arc<BinanceWebSocketConnector>,
        connection: Arc<Mutex<BinanceWebSocketConnection>>,
        cmd_ch_size: usize,
        remove_id_ch_size: usize,
    ) -> Self {
        let (command_tx, command_rx) = channel::<Command>(cmd_ch_size);
        let (removed_ids_tx, removed_ids_rx) = channel::<Vec<String>>(remove_id_ch_size);

        let cmd_tx = Arc::new(command_tx);

        let cache = Arc::new(Cache::new(Some(removed_ids_tx)));

        start_service(
            connector,
            connection,
            command_rx,
            removed_ids_rx,
            cache.clone(),
            cmd_tx.clone(),
        );

        Self { cache, cmd_tx }
    }
}

#[async_trait::async_trait]
impl Service for BinanceService {
    async fn get_price_data(&mut self, ids: &[&str]) -> Vec<ServiceResult<PriceData>> {
        let mut sub_ids = Vec::new();

        let result = self
            .cache
            .get_batch(ids)
            .await
            .into_iter()
            .enumerate()
            .map(|(idx, result)| match result {
                Ok(price_data) => Ok(price_data),
                Err(CacheError::DoesNotExist) => {
                    // If the id is not in the cache, subscribe to it
                    sub_ids.push(ids[idx].to_string());
                    Err(ServiceError::Pending)
                }
                Err(CacheError::Invalid) => Err(ServiceError::InvalidSymbol),
                Err(e) => panic!("unexpected error: {}", e), // This should never happen
            })
            .collect();

        if !sub_ids.is_empty() && self.cmd_tx.send(Command::Subscribe(sub_ids)).await.is_err() {
            warn!("Failed to send subscribe command");
        }

        result
    }
}

fn start_service(
    connector: Arc<BinanceWebSocketConnector>,
    connection: Arc<Mutex<BinanceWebSocketConnection>>,
    mut command_rx: Receiver<Command>,
    mut removed_ids_rx: Receiver<Vec<String>>,
    cache: Arc<Cache<PriceData>>,
    command_tx: Arc<Sender<Command>>,
) {
    tokio::spawn(async move {
        loop {
            select! {
                Some(cmd) = command_rx.recv() => {
                    process_command(&cmd, &connection, &cache).await;
                },
                result = {
                    let cloned = connection.clone();
                    timeout(DEFAULT_TIMEOUT, async move { cloned.lock().await.next().await })
                } => {
                    match &result {
                        Ok(Ok(result)) => {
                            process_response(result, &cache).await;
                        },
                        Ok(Err(BinanceError::ChannelClosed)) | Err(_) => {
                            // Attempt to reconnect on timeout or on channel close
                            handle_reconnect(&connector, &connection, &cache, &command_tx).await;
                        }
                        Ok(Err(e)) => {
                            error!("unexpected error: {}", e);
                        }
                    }
                },
                Some(ids) = removed_ids_rx.recv() => {
                    let vec_ids = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                    if connection.lock().await.unsubscribe(vec_ids.as_slice()).await.is_err() {
                        warn!("failed to unsubscribe to ids: {:?}", ids);
                    }
                }
            }
        }
    });
}

async fn process_command(
    cmd: &Command,
    ws: &Mutex<BinanceWebSocketConnection>,
    cache: &Cache<PriceData>,
) {
    match cmd {
        Command::Subscribe(ids) => {
            cache.set_batch_pending(ids.clone()).await;

            let vec_ids = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
            let mut locked = ws.lock().await;
            if locked.subscribe(vec_ids.as_slice()).await.is_err() {
                warn!("Failed to subscribe to ids: {:?}", ids);
            }
        }
    }
}

async fn handle_reconnect(
    connector: &BinanceWebSocketConnector,
    connection: &Mutex<BinanceWebSocketConnection>,
    cache: &Cache<PriceData>,
    command_tx: &Sender<Command>,
) {
    // TODO: Handle reconnection failure
    let mut locked = connection.lock().await;
    warn!("attempting to reconnect to binance");
    // reconnect
    if let Ok(new_connection) = connector.connect().await {
        *locked = new_connection;
    } else {
        warn!("failed to reconnect to binance");
        return;
    }

    //  resubscribe to all ids
    let keys = cache.keys().await;
    if !keys.is_empty() && command_tx.send(Command::Subscribe(keys)).await.is_err() {
        error!("Failed to send subscribe command");
    };
}

async fn save_datum(data: &Data, cache: &Cache<PriceData>) {
    match data {
        Data::MiniTicker(ticker) => {
            let price_data = PriceData {
                id: ticker.symbol.clone(),
                price: ticker.close_price.clone().to_string(),
                timestamp: ticker.event_time,
            };
            info!("received prices: {:?}", price_data);
            let id = price_data.id.clone();
            if cache
                .set_data(ticker.symbol.clone(), price_data)
                .await
                .is_err()
            {
                warn!("unexpected request to set data for id: {}", id);
            } else {
                info!("set price for id {}", id);
            }
        }
    }
}

async fn process_response(resp: &BinanceResponse, cache: &Cache<PriceData>) {
    match resp {
        BinanceResponse::Stream(resp) => save_datum(&resp.data, cache).await,
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

#[cfg(test)]
mod test {
    use tokio_tungstenite::tungstenite::Message;
    use ws_mock::ws_mock_server::WsMock;

    use crate::api::types::{Data, MiniTickerInfo, StreamResponse};
    use crate::api::websocket::test::setup_mock_server;

    use super::*;

    #[tokio::test]
    async fn test_process_command() {
        let mock = setup_mock_server().await;

        let connector = Arc::new(BinanceWebSocketConnector::new(mock.uri().await));
        let connection = Arc::new(Mutex::new(connector.connect().await.unwrap()));

        let cache = Arc::new(Cache::new(None));

        let (_, msg_rx) = channel::<Message>(32);
        WsMock::new()
            .forward_from_channel(msg_rx)
            .mount(&mock)
            .await;

        process_command(
            &Command::Subscribe(vec!["btcusdt".to_string()]),
            &connection,
            &cache,
        )
        .await;
    }

    #[tokio::test]
    async fn test_save_datum() {
        let cache = Arc::new(Cache::new(None));
        cache.set_pending("btcusdt".to_string()).await;

        let ticker = MiniTickerInfo {
            event_time: 1628794647025,
            symbol: "btcusdt".to_string(),
            close_price: "45000.00".to_string(),
            open_price: "44000.00".to_string(),
            high_price: "46000.00".to_string(),
            low_price: "43000.00".to_string(),
            base_volume: "1000.00".to_string(),
            quote_volume: "45000000.00".to_string(),
        };

        let data = Data::MiniTicker(ticker);
        save_datum(&data, &cache).await;

        let price_data = cache.get("btcusdt").await.unwrap();
        assert_eq!(
            price_data,
            PriceData {
                id: "btcusdt".to_string(),
                price: "45000.00".to_string(),
                timestamp: 1628794647025,
            }
        );
    }

    #[tokio::test]
    async fn test_process_response() {
        let cache = Arc::new(Cache::new(None));
        cache.set_pending("btcusdt".to_string()).await;

        let ticker = MiniTickerInfo {
            event_time: 1628794647025,
            symbol: "btcusdt".to_string(),
            close_price: "45000.00".to_string(),
            open_price: "44000.00".to_string(),
            high_price: "46000.00".to_string(),
            low_price: "43000.00".to_string(),
            base_volume: "1000.00".to_string(),
            quote_volume: "45000000.00".to_string(),
        };

        let data = Data::MiniTicker(ticker);
        let stream_resp = StreamResponse {
            stream: "btc@miniTicker".to_string(),
            data,
        };

        let response = BinanceResponse::Stream(stream_resp);
        process_response(&response, &cache).await;

        let price_data = cache.get("btcusdt").await.unwrap();
        assert_eq!(
            price_data,
            PriceData {
                id: "btcusdt".to_string(),
                price: "45000.00".to_string(),
                timestamp: 1628794647025,
            }
        );
    }
}
