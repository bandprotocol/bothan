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
}

impl BinanceWebsocketService {
    async fn get_prices(&self, symbol_ids: &[(&str, &str)]) -> Vec<Result<PriceInfo, Error>> {
        let mut prices = Vec::new();
        let locked_cached_price = self.cached_price.lock().await;

        for (base, quote) in symbol_ids {
            let symbol = format!("{}/{}", base, quote);

            let price = match locked_cached_price.get(&symbol) {
                Some(price) => Ok(price.clone()),
                None => Err(Error::NotFound(symbol)),
            };

            prices.push(price);
        }

        prices
    }

    pub async fn get_price(&self, base: &str, quote: &str) -> Result<PriceInfo, Error> {
        let symbol_ids = vec![(base, quote)];
        let mut prices = self.get_prices(&symbol_ids).await;

        prices
            .pop()
            .ok_or(Error::NotFound(format!("{}/{}", base, quote)))?
    }
}
