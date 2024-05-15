use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tracing::{error, info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::error::Error as CoinbaseError;
use crate::api::types::channels::ticker::Ticker;
use crate::api::types::channels::Channel;
use crate::api::types::CoinbaseResponse;
use crate::types::{Command, DEFAULT_TIMEOUT};
use crate::{CoinbaseWebSocketConnection, CoinbaseWebSocketConnector};

pub mod builder;

pub struct CoinbaseService {
    cache: Arc<Cache<PriceData>>,
    cmd_tx: Arc<Sender<Command>>,
}

impl CoinbaseService {
    pub fn new(
        connector: Arc<CoinbaseWebSocketConnector>,
        connection: Arc<Mutex<CoinbaseWebSocketConnection>>,
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
impl Service for CoinbaseService {
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

fn start_service(
    connector: Arc<CoinbaseWebSocketConnector>,
    connection: Arc<Mutex<CoinbaseWebSocketConnection>>,
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
                        Ok(Ok(response)) => {
                            process_response(response, &cache).await;
                        },
                        Ok(Err(CoinbaseError::ChannelClosed)) | Err(_) => {
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
                    if connection.lock().await.unsubscribe(vec![Channel::Ticker], vec_ids.as_slice()).await.is_err() {
                        warn!("failed to unsubscribe to ids: {:?}", ids);
                    }
                }
            }
        }
    });
}

async fn process_command(
    cmd: &Command,
    ws: &Mutex<CoinbaseWebSocketConnection>,
    cache: &Cache<PriceData>,
) {
    match cmd {
        Command::Subscribe(ids) => {
            cache.set_batch_pending(ids.clone()).await;

            let vec_ids = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
            let mut locked = ws.lock().await;
            if locked
                .subscribe(vec![Channel::Ticker], vec_ids.as_slice())
                .await
                .is_err()
            {
                warn!("Failed to subscribe to ids: {:?}", ids);
            }
        }
    }
}

async fn handle_reconnect(
    connector: &CoinbaseWebSocketConnector,
    connection: &Mutex<CoinbaseWebSocketConnection>,
    cache: &Cache<PriceData>,
    command_tx: &Sender<Command>,
) {
    // TODO: Handle reconnection failure
    let mut locked = connection.lock().await;
    warn!("attempting to reconnect to coinbase");
    // reconnect
    if let Ok(new_connection) = connector.connect().await {
        *locked = new_connection;
    } else {
        warn!("failed to reconnect to coinbase");
        return;
    }

    //  resubscribe to all ids
    let keys = cache.keys().await;

    // If the keys are empty, subscribe to a single symbol to keep the connection alive
    // TODO: find a better solution
    let cmd = match keys.is_empty() {
        true => Command::Subscribe(vec!["BTC-USD".to_string()]),
        false => Command::Subscribe(keys),
    };

    if command_tx.send(cmd).await.is_err() {
        error!("Failed to send subscribe command");
    };
}

async fn save_ticker(ticker: &Ticker, cache: &Cache<PriceData>) {
    if let Ok(date_time) = chrono::DateTime::parse_from_rfc3339(&ticker.time) {
        let price_data = PriceData {
            id: ticker.product_id.clone(),
            price: ticker.price.clone(),
            timestamp: date_time.timestamp() as u64,
        };
        info!("received prices: {:?}", price_data);
        let id = price_data.id.clone();
        if cache.set_data(id.clone(), price_data).await.is_err() {
            warn!("unexpected request to set data for id: {}", id);
        } else {
            info!("set price for id {}", id);
        }
    } else {
        warn!(
            "failed to parse date time for ticker {}: {}",
            ticker.product_id, ticker.time
        );
    }
}

async fn process_response(resp: &CoinbaseResponse, cache: &Cache<PriceData>) {
    match resp {
        CoinbaseResponse::Ticker(ticker) => save_ticker(ticker, cache).await,
        CoinbaseResponse::Subscriptions(_) => {
            info!("received request response");
        }
    }
}
