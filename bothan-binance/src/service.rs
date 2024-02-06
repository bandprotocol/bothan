use reqwest::Url;
use std::sync::Arc;
use std::time::Duration;

use tokio::select;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::{interval, timeout, MissedTickBehavior};
use tracing::{error, info, warn};

use crate::api::error::Error as BinanceError;
use crate::api::types::{BinanceResponse, Data};
use crate::api::websocket::BinanceWebsocket;
use crate::cache::{Cache, Error as CacheError};
use crate::error::Error;
use crate::types::PriceData;

pub const DEFAULT_CHANNEL_SIZE: usize = 100;

enum Command {
    Subscribe(Vec<String>),
}

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

        let mut timeout_interval = interval(Duration::new(120, 0));
        // consume the first tick to start the interval
        timeout_interval.tick().await;
        timeout_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        tokio::spawn(async move {
            loop {
                select! {
                    Some(cmd) = command_rx.recv() => {
                        process_command(&cmd, &mut ws, &cloned_cache).await;
                    },
                    result = timeout(Duration::new(120, 0), ws.next()) => {
                        match &result {
                            Ok(result) => {
                                process_results(result, &cloned_cache).await;
                            },
                            Err(_) => {
                                handle_reconnect(&mut ws, &cloned_cache, &cloned_command_tx).await;
                            }
                        }
                    },
                    Some(ids) = removed_ids_rx.recv() => {
                        // TODO: this may reinit data that is meant to be removed if
                        // a packet comes in before the unsubscribe is completed. Need
                        // to resolve this later
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
        let result = ids
            .iter()
            .map(|id| match self.cache.get(id) {
                Ok(price_data) => Ok(price_data),
                Err(CacheError::DoesNotExist) => {
                    // If the id is not in the cache, subscribe to it
                    sub_ids.push(id.to_string());
                    Err(Error::Pending)
                }
                Err(CacheError::Invalid) => Err(Error::Unknown),
            })
            .collect::<Vec<Result<PriceData, Error>>>();

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
                cloned_cache.set_pending(id.clone());
            }
        }
    }
}

async fn handle_reconnect(
    ws: &mut BinanceWebsocket,
    cache: &Arc<Cache>,
    command_tx: &Sender<Command>,
) {
    error!("timeout waiting for response from binance, attempting to reconnect");
    // reconnect
    _ = ws.disconnect().await;
    _ = ws.connect().await;
    //  resubscribe to all ids
    let keys = cache.keys();
    if !keys.is_empty() && command_tx.send(Command::Subscribe(keys)).await.is_err() {
        warn!("Failed to send subscribe command");
    };
}

async fn process_results(results: &Result<BinanceResponse, BinanceError>, cache: &Arc<Cache>) {
    match results {
        Ok(BinanceResponse::Stream(resp)) => match &resp.data {
            Data::MiniTicker(ticker) => {
                let price_data = PriceData {
                    id: ticker.symbol.clone(),
                    price: ticker.close_price.clone().to_string(),
                    timestamp: ticker.event_time,
                };
                info!("received prices: {:?}", price_data);
                cache.set_data(ticker.symbol.clone(), price_data);
            }
        },
        Ok(BinanceResponse::Success(_)) => {
            // TODO: better logging
            info!("subscribed to ids");
        }
        Ok(BinanceResponse::Error(_)) => {
            // TODO: better logging
            error!("error received from binance");
        }
        Err(e) => {
            error!("unable able to parse message from binance: {:?}", e);
        }
    }
}
