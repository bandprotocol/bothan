use futures_util::StreamExt;
use std::{collections::HashMap, sync::Arc};
use tokio::{select, sync::Mutex};
use tokio_util::sync::CancellationToken;

use super::websocket::BinanceWebsocket;
use crate::{error::Error, types::PriceInfo};

/// A caching object storing prices received from binance websocket.
pub struct BinanceWebsocketService {
    socket: Arc<Mutex<BinanceWebsocket>>,
    cached_price: Arc<Mutex<HashMap<String, PriceInfo>>>,
    cancellation_token: Option<CancellationToken>,
}

impl BinanceWebsocketService {
    /// initiate new object from created socket.
    pub fn new(socket: BinanceWebsocket) -> Self {
        Self {
            socket: Arc::new(Mutex::new(socket)),
            cached_price: Arc::new(Mutex::new(HashMap::new())),
            cancellation_token: None,
        }
    }

    /// start a service.
    pub fn start(&mut self) -> Result<(), Error> {
        self.stop();

        let token = CancellationToken::new();
        let cloned_token = token.clone();
        let cloned_socket = Arc::clone(&self.socket);
        let cloned_cached_price = Arc::clone(&self.cached_price);
        self.cancellation_token = Some(token);

        tokio::spawn(async move {
            loop {
                let mut locked_socket = cloned_socket.lock().await;
                select! {
                    _ = cloned_token.cancelled() => {
                        break;
                    }

                    result = locked_socket.next() => {
                        drop(locked_socket);

                        match result {
                            Some(Ok(price_info)) => {
                                let mut locked_cached_price = cloned_cached_price.lock().await;
                                locked_cached_price.insert(price_info.id.to_string(), price_info);
                            }
                            Some(Err(err)) => {
                                tracing::error!("cannot get price: {}", err);
                            }
                            None => {
                                tracing::error!("cannot get price: stream ended");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// stop a service.
    pub fn stop(&mut self) {
        if let Some(token) = &self.cancellation_token {
            token.cancel();
        }
    }

    pub async fn get_prices(&self, ids: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        let mut prices = Vec::new();
        let locked_cached_price = self.cached_price.lock().await;

        for &id in ids {
            let price = match locked_cached_price.get(&id.to_ascii_uppercase()) {
                Some(price) => Ok(price.clone()),
                None => Err(Error::NotFound(id.to_string())),
            };

            prices.push(price);
        }

        prices
    }
}
