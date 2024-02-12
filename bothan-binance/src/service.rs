use std::sync::Arc;
use std::time::Duration;

use tokio::select;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::timeout;
use tracing::{error, info, warn};

use crate::api::error::Error as BinanceError;
use crate::api::types::{BinanceResponse, Data};
use crate::api::websocket::BinanceWebsocket;
use crate::cache::{Cache, Error as CacheError};
use crate::error::Error;
use crate::types::{Command, PriceData};

pub const DEFAULT_CHANNEL_SIZE: usize = 100;

pub struct BinanceService {
    cache: Arc<Cache>,
    cmd_tx: Arc<Sender<Command>>,
}

impl BinanceService {
    pub async fn new(url: Option<&str>) -> Result<Self, Error> {
        let mut ws = get_websocket(url);
        ws.connect().await?;

        let (command_tx, mut command_rx) = channel::<Command>(DEFAULT_CHANNEL_SIZE);
        let (removed_ids_tx, mut removed_ids_rx) = channel::<Vec<String>>(DEFAULT_CHANNEL_SIZE);

        let command_tx = Arc::new(command_tx);
        let cloned_command_tx = command_tx.clone();

        let cache = Arc::new(Cache::new(removed_ids_tx));
        let cloned_cache = cache.clone();

        tokio::spawn(async move {
            loop {
                select! {
                    Some(cmd) = command_rx.recv() => {
                        process_command(&cmd, &mut ws, &cloned_cache).await;
                    },
                    result = timeout(Duration::new(120, 0), ws.next()) => {
                        match &result {
                            Ok(result) => {
                                process_result(result, &cloned_cache).await;
                            },
                            Err(_) => {
                                handle_reconnect(&mut ws, &cloned_cache, &cloned_command_tx).await;
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

        Ok(Self {
            cache,
            cmd_tx: command_tx,
        })
    }

    pub async fn get_price_data(&mut self, ids: &[&str]) -> Vec<Result<PriceData, Error>> {
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
                    Err(Error::Pending)
                }
                Err(CacheError::Invalid) => Err(Error::InvalidSymbol),
                Err(e) => panic!("unexpected error: {}", e), // This should never happen
            })
            .collect();

        if !sub_ids.is_empty() && self.cmd_tx.send(Command::Subscribe(sub_ids)).await.is_err() {
            warn!("Failed to send subscribe command");
        }

        result
    }
}

fn get_websocket(url: Option<&str>) -> BinanceWebsocket {
    if let Some(endpoint) = url {
        BinanceWebsocket::new(endpoint)
    } else {
        BinanceWebsocket::default()
    }
}

async fn process_command(cmd: &Command, ws: &mut BinanceWebsocket, cloned_cache: &Arc<Cache>) {
    match cmd {
        Command::Subscribe(ids) => {
            let vec_ids = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
            if ws.subscribe(vec_ids.as_slice()).await.is_err() {
                warn!("Failed to subscribe to ids: {:?}", ids);
            }

            for id in ids {
                if cloned_cache.set_pending(id.clone()).await.is_err() {
                    warn!("Failed to set pending for id: {}", id);
                }
            }
        }
    }
}

async fn handle_reconnect(
    ws: &mut BinanceWebsocket,
    cache: &Arc<Cache>,
    command_tx: &Sender<Command>,
) {
    warn!("timeout waiting for response from binance, attempting to reconnect");
    // reconnect
    _ = ws.disconnect().await;

    // TODO: handle this if reconnect attempt fails
    _ = ws.connect().await;

    //  resubscribe to all ids
    let keys = cache.keys().await;
    if !keys.is_empty() && command_tx.send(Command::Subscribe(keys)).await.is_err() {
        error!("Failed to send subscribe command");
    };
}

async fn save_datum(data: &Data, cache: &Arc<Cache>) {
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

async fn process_result(result: &Result<BinanceResponse, BinanceError>, cache: &Arc<Cache>) {
    match result {
        Ok(BinanceResponse::Stream(resp)) => save_datum(&resp.data, cache).await,
        Ok(BinanceResponse::Success(_)) => {
            info!("subscription success");
        }
        Ok(BinanceResponse::Error(e)) => {
            error!("error code {} received from binance: {}", e.code, e.msg);
        }
        Err(e) => {
            error!("unable able to parse message from binance: {:?}", e);
        }
    }
}
