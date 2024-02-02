use std::sync::Arc;

use tokio::select;
use tokio::sync::mpsc::{channel, Sender};
use tracing::{info, warn};

use crate::cache::{Cache, Error as CacheError};
use crate::error::Error;
use crate::types::PriceData;
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
        // TODO: do hardcoded ch size later
        let (command_tx, mut command_rx) = channel::<Command>(DEFAULT_CHANNEL_SIZE);
        let (removed_ids_tx, mut removed_ids_rx) = channel::<Vec<String>>(DEFAULT_CHANNEL_SIZE);

        let command_tx = Arc::new(command_tx);
        let cache = Arc::new(Cache::new(removed_ids_tx));

        let cloned_cache = cache.clone();
        tokio::spawn(async move {
            loop {
                select! {
                    Some(cmd) = command_rx.recv() => {
                        match &cmd {
                            Command::Subscribe(ids) => {
                                let dd = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                                for id in ids {
                                    cloned_cache.set_pending(id.clone());
                                }
                                if let Err(_) = ws.subscribe(dd.as_slice()).await {
                                    warn!("Failed to subscribe to ids: {:?}", ids);
                                }
                            }
                        }
                    },
                    Ok(ticker) = ws.next() => {
                        info!("Received ticker: {:?}", ticker);
                        let price_data = PriceData {
                            id: ticker.id.clone(),
                            price: ticker.current_price.clone(),
                            timestamp: ticker.timestamp.clone()
                        };
                        cloned_cache.set_data(ticker.id.clone(), price_data);
                    },
                    Some(ids) = removed_ids_rx.recv() => {
                        // TODO: this may reinit data that is meant to be removed if
                        // a packet comes in before the unsubscribe is completed. Need
                        // to resolve this later
                        let dd = ids.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                        if let Err(_) = ws.unsubscribe(dd.as_slice()).await {
                            warn!("Failed to unsubscribe to ids: {:?}", ids);
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

        if ids_to_subscribe.len() > 0 {
            if let Err(_) = self
                .command_tx
                .send(Command::Subscribe(ids_to_subscribe))
                .await
            {
                warn!("Failed to send subscribe command");
            }
        }

        result
    }
}
