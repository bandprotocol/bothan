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
    pub async fn start(&mut self) -> Result<(), Error> {
        if self.cancellation_token.is_some() {
            return Err(Error::AlreadyStarted);
        }

        let mut locked_socket = self.socket.lock().await;
        if !locked_socket.is_connected() {
            locked_socket.connect().await?;
        }
        drop(locked_socket);

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
                                tracing::trace!("cannot get price: {}", err);
                            }
                            None => {
                                tracing::trace!("cannot get price: stream ended");
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
        self.cancellation_token = None;
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
