use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::{interval, timeout};
use tracing::{debug, error, info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::error::Error as KrakenError;
use crate::api::types::channel::ticker::TickerResponse;
use crate::api::types::channel::ChannelResponse;
use crate::api::types::KrakenResponse;
use crate::types::{Command, DEFAULT_TIMEOUT, KEEP_ALIVE_INTERVAL};
use crate::{KrakenWebSocketConnection, KrakenWebSocketConnector};

pub mod builder;

/// Service for interacting with Kraken WebSocket API and managing price data cache.
pub struct KrakenService {
    cache: Arc<Cache<PriceData>>,
    cmd_tx: Arc<Sender<Command>>,
}

impl KrakenService {
    /// Creates a new `KrakenService` instance.
    pub fn new(
        connector: Arc<KrakenWebSocketConnector>,
        connection: Arc<Mutex<KrakenWebSocketConnection>>,
        cmd_ch_size: usize,
        rem_id_ch_size: usize,
    ) -> Self {
        let (command_tx, command_rx) = channel::<Command>(cmd_ch_size);
        let (removed_ids_tx, removed_ids_rx) = channel::<Vec<String>>(rem_id_ch_size);

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
impl Service for KrakenService {
    /// Retrieves price data for the given IDs.
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
                    Err(ServiceError::PendingResult)
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

/// Starts the service for updating price data.
fn start_service(
    connector: Arc<KrakenWebSocketConnector>,
    connection: Arc<Mutex<KrakenWebSocketConnection>>,
    mut command_rx: Receiver<Command>,
    mut removed_ids_rx: Receiver<Vec<String>>,
    cache: Arc<Cache<PriceData>>,
    command_tx: Arc<Sender<Command>>,
) {
    tokio::spawn(async move {
        let mut ping_interval = interval(KEEP_ALIVE_INTERVAL);
        loop {
            select! {
                _ = ping_interval.tick() => {
                    if connection.lock().await.ping().await.is_err() {
                        warn!("failed to ping kraken");
                    } else {
                        debug!("sent ping to kraken");
                    }
                },
                Some(cmd) = command_rx.recv() => {
                    process_command(&cmd, &connection, &cache).await;
                },
                result = {
                    let cloned = connection.clone();
                    timeout(DEFAULT_TIMEOUT, async move { cloned.lock().await.next().await })
                } => {
                    let timestamp = chrono::Utc::now().timestamp();
                    match &result {
                        Ok(Ok(response)) => {
                            process_response(response, &cache, timestamp as u64).await;
                        },
                        Ok(Err(KrakenError::ChannelClosed)) | Err(_) => {
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
                    if connection.lock().await.unsubscribe_ticker(vec_ids.as_slice()).await.is_err() {
                        warn!("failed to unsubscribe to ids: {:?}", ids);
                    }
                }
            }
        }
    });
}

/// Processes the command for subscribing to tickers.
async fn process_command(
    cmd: &Command,
    ws: &Mutex<KrakenWebSocketConnection>,
    cache: &Cache<PriceData>,
) {
    match cmd {
        Command::Subscribe(ids) => {
            cache.set_batch_pending(ids.clone()).await;

            let vec_ids = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
            let mut locked = ws.lock().await;
            if locked
                .subscribe_ticker(vec_ids.as_slice(), None, None)
                .await
                .is_err()
            {
                warn!("Failed to subscribe to ids: {:?}", ids);
            }
        }
    }
}

/// Handles the reconnection logic.
async fn handle_reconnect(
    connector: &KrakenWebSocketConnector,
    connection: &Mutex<KrakenWebSocketConnection>,
    cache: &Cache<PriceData>,
    command_tx: &Sender<Command>,
) {
    // TODO: Handle reconnection failure
    let mut locked = connection.lock().await;
    warn!("attempting to reconnect to kraken");
    // reconnect
    if let Ok(new_connection) = connector.connect().await {
        *locked = new_connection;
    } else {
        warn!("failed to reconnect to kraken");
        return;
    }

    //  resubscribe to all ids
    let keys = cache.keys().await;
    if !keys.is_empty() && command_tx.send(Command::Subscribe(keys)).await.is_err() {
        error!("Failed to send subscribe command");
    };
}

/// Saves the ticker data to the cache.
async fn save_tickers(tickers: &Vec<TickerResponse>, cache: &Cache<PriceData>, timestamp: u64) {
    for ticker in tickers {
        let id = ticker.symbol.clone();
        let price_data = PriceData {
            id: id.clone(),
            price: ticker.last.clone().to_string(),
            timestamp,
        };

        info!("received prices: {:?}", price_data);
        if cache.set_data(id.clone(), price_data).await.is_err() {
            warn!("unexpected request to set data for id: {}", id);
        } else {
            info!("set price for id {}", id);
        }
    }
}

/// Processes the response from the Kraken API.
async fn process_response(resp: &KrakenResponse, cache: &Cache<PriceData>, timestamp: u64) {
    match resp {
        KrakenResponse::Channel(resp) => match resp {
            ChannelResponse::Ticker(tickers) => save_tickers(tickers, cache, timestamp).await,
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
