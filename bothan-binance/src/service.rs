use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use bothan_core::cache::{Cache, Error as CacheError};
use bothan_core::service::{Error as ServiceError, Service, ServiceResult};
use bothan_core::types::PriceData;

use crate::api::error::Error as BinanceError;
use crate::api::types::{BinanceResponse, Data};
use crate::api::websocket::BinanceWebsocket;
use crate::error::Error;
use crate::types::{Command, DEFAULT_CHANNEL_SIZE, DEFAULT_TIMEOUT};

pub struct Binance {
    cache: Arc<Cache<PriceData>>,
    cmd_tx: Arc<Sender<Command>>,
}

impl Binance {
    pub async fn new(
        url: impl Into<String>,
        cmd_ch_size: usize,
        rem_id_ch_size: usize,
    ) -> Result<Self, Error> {
        Self::_new(BinanceWebsocket::new(url), cmd_ch_size, rem_id_ch_size).await
    }

    pub async fn default() -> Result<Self, Error> {
        Self::_new(
            BinanceWebsocket::default(),
            DEFAULT_CHANNEL_SIZE,
            DEFAULT_CHANNEL_SIZE,
        )
        .await
    }

    async fn _new(
        mut ws: BinanceWebsocket,
        cmd_ch_size: usize,
        rem_id_ch_size: usize,
    ) -> Result<Self, Error> {
        ws.connect().await?;

        let (command_tx, command_rx) = channel::<Command>(cmd_ch_size);
        let (removed_ids_tx, removed_ids_rx) = channel::<Vec<String>>(rem_id_ch_size);

        let cmd_tx = Arc::new(command_tx);

        let cache = Arc::new(Cache::new(Some(removed_ids_tx)));

        start_service(
            ws,
            command_rx,
            removed_ids_rx,
            cache.clone(),
            cmd_tx.clone(),
        );

        Ok(Self { cache, cmd_tx })
    }
}

#[async_trait::async_trait]
impl Service for Binance {
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
    mut ws: BinanceWebsocket,
    mut command_rx: Receiver<Command>,
    mut removed_ids_rx: Receiver<Vec<String>>,
    cache: Arc<Cache<PriceData>>,
    arced_command_tx: Arc<Sender<Command>>,
) {
    tokio::spawn(async move {
        loop {
            select! {
                Some(cmd) = command_rx.recv() => {
                    process_command(&cmd, &mut ws, &cache).await;
                },
                result = timeout(DEFAULT_TIMEOUT, ws.next()) => {
                    match &result {
                        Ok(Ok(result)) => {
                            process_response(result, &cache).await;
                        },
                        Ok(Err(BinanceError::ChannelClosed)) | Err(_) => {
                            // Attempt to reconnect on timeout or on channel close
                            handle_reconnect(&mut ws, &cache, &arced_command_tx).await;
                        }
                        Ok(Err(e)) => {
                            error!("unexpected error: {}", e);
                        }
                    }
                },
                Some(ids) = removed_ids_rx.recv() => {
                    let vec_ids = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                    if ws.unsubscribe(vec_ids.as_slice()).await.is_err() {
                        warn!("failed to unsubscribe to ids: {:?}", ids);
                    }
                }
            }
        }
    });
}

async fn process_command(cmd: &Command, ws: &mut BinanceWebsocket, cache: &Arc<Cache<PriceData>>) {
    match cmd {
        Command::Subscribe(ids) => {
            cache.set_batch_pending(ids.clone()).await;

            let vec_ids = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
            if ws.subscribe(vec_ids.as_slice()).await.is_err() {
                warn!("Failed to subscribe to ids: {:?}", ids);
            }
        }
    }
}

async fn handle_reconnect(
    ws: &mut BinanceWebsocket,
    cache: &Arc<Cache<PriceData>>,
    command_tx: &Sender<Command>,
) {
    warn!("attempting to reconnect to binance");
    // reconnect
    _ = ws.disconnect().await;

    // TODO: handle this if reconnect attempt fails
    let result = ws.connect().await;
    if result.is_err() {
        error!("failed to reconnect to binance");
    } else {
        info!("reconnected to binance")
    }

    //  resubscribe to all ids
    let keys = cache.keys().await;
    if !keys.is_empty() && command_tx.send(Command::Subscribe(keys)).await.is_err() {
        error!("Failed to send subscribe command");
    };
}

async fn save_datum(data: &Data, cache: &Arc<Cache<PriceData>>) {
    match data {
        Data::MiniTicker(ticker) => {
            let price_data = PriceData {
                id: ticker.symbol.clone(),
                price: ticker.close_price.clone().to_string(),
                timestamp: ticker.event_time,
            };
            info!("received prices: {:?}", price_data);
            match cache.set_data(ticker.symbol.clone(), price_data).await {
                Ok(_) => {
                    info!("successfully set {} in cache", ticker.symbol);
                }
                Err(CacheError::PendingNotSet) => {
                    warn!(
                        "received data for id that was not pending: {}",
                        ticker.symbol
                    );
                }
                Err(e) => {
                    error!("error setting data in cache: {:?}", e)
                }
            }
        }
    }
}

async fn process_response(resp: &BinanceResponse, cache: &Arc<Cache<PriceData>>) {
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
