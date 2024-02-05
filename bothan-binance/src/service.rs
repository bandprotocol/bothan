use std::sync::Arc;
use std::time::Duration;

use tokio::select;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::{interval, timeout, MissedTickBehavior};
use tracing::{error, info, warn};

use crate::cache::{Cache, Error as CacheError};
use crate::error::Error;
use crate::types::PriceData;
use crate::websocket::types::{BinanceResponse, Data};
use crate::websocket::BinanceWebsocket;

pub const DEFAULT_CHANNEL_SIZE: usize = 100;

enum Command {
    Subscribe(Vec<String>),
}

pub struct BinanceService {
    cache: Arc<Cache>,
    command_tx: Arc<Sender<Command>>,
}

impl BinanceService {
    pub async fn new(mut ws: BinanceWebsocket) -> Result<Self, Error> {
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

        // TODO: need to implement reconnect in case of binance disconnect
        tokio::spawn(async move {
            loop {
                select! {
                    Some(cmd) = command_rx.recv() => {
                        match &cmd {
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
                    },
                    result = timeout(Duration::new(120, 0), ws.next()) => {
                        match &result {
                            Ok(binance_result) => {
                                match binance_result {
                                    Ok(BinanceResponse::Stream(resp)) => {
                                        match &resp.data {
                                            Data::MiniTicker(ticker) => {
                                                let price_data = PriceData {
                                                    id: ticker.symbol.clone(),
                                                    price: ticker.close_price.clone().to_string(),
                                                    timestamp: ticker.event_time.clone()
                                                };
                                                info!("received prices: {:?}", price_data);
                                                cloned_cache.set_data(ticker.symbol.clone(), price_data);
                                            },
                                        }
                                    }
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
                            },
                            Err(e) => {
                                error!("timeout waiting for response from binance, attempting to reconnect");
                                _ = ws.disconnect().await;
                                _ = ws.connect().await;
                                // resub
                                let keys = cloned_cache.keys();
                                if !keys.is_empty() {
                                    if let Err(_) = cloned_command_tx.send(Command::Subscribe(keys)).await {
                                        warn!("Failed to send subscribe command");
                                    }
                                }
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
        Ok(Self { cache, command_tx })
    }

    pub async fn get_price_data(&mut self, ids: &[&str]) -> Vec<Result<PriceData, Error>> {
        let mut result = Vec::new();
        let mut ids_to_subscribe = Vec::new();

        for id in ids {
            match self.cache.get(id) {
                Ok(price_data) => result.push(Ok(price_data)),
                Err(CacheError::DoesNotExist) => {
                    // If the id is not in the cache, subscribe to it
                    ids_to_subscribe.push(id.to_string());
                    result.push(Err(Error::Pending))
                }
                Err(CacheError::Invalid) => {
                    result.push(Err(Error::Unknown));
                }
            }
        }

        if ids_to_subscribe.len() > 0
            && self
                .command_tx
                .send(Command::Subscribe(ids_to_subscribe))
                .await
                .is_err()
        {
            warn!("Failed to send subscribe command");
        }

        result
    }
}
