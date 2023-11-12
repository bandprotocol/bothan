use futures_util::StreamExt;
use std::{collections::HashMap, sync::Arc};
use tokio::{select, sync::Mutex};
use tokio_util::sync::CancellationToken;

use super::websocket::BinanceWebsocket;
use crate::{error::Error, types::PriceInfo};

pub struct BinanceWebsocketService {
    socket: Arc<Mutex<BinanceWebsocket>>,
    cached_price: Arc<Mutex<HashMap<String, PriceInfo>>>,
    cancellation_token: Option<CancellationToken>,
}

impl BinanceWebsocketService {
    pub fn new(socket: BinanceWebsocket) -> Self {
        Self {
            socket: Arc::new(Mutex::new(socket)),
            cached_price: Arc::new(Mutex::new(HashMap::new())),
            cancellation_token: None,
        }
    }

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
                                let symbol = format!("{}/{}", price_info.base, price_info.quote);
                                locked_cached_price.insert(symbol, price_info);
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

    pub fn stop(&mut self) {
        if let Some(token) = &self.cancellation_token {
            token.cancel();
        }
    }

    pub async fn get_price(&mut self, base: &str, quote: &str) -> Result<PriceInfo, Error> {
        let symbol = format!("{}/{}", base, quote);

        let locked_cached_price = self.cached_price.lock().await;
        if let Some(price) = locked_cached_price.get(&symbol) {
            Ok(price.clone())
        } else {
            Err(Error::NotFound(symbol))
        }
    }
}
