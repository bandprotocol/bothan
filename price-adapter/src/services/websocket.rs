use crate::{
    error::Error,
    types::{PriceInfo, Service, Source, WebSocketSource, WebsocketMessage},
};
use std::{collections::HashMap, sync::Arc};
use tokio::{select, sync::Mutex};
use tokio_util::sync::CancellationToken;

/// A caching object storing prices received from WebSocketSource.
pub struct WebsocketService<S: WebSocketSource> {
    socket: Arc<Mutex<S>>,
    cached_prices: Arc<Mutex<HashMap<String, PriceInfo>>>,
    cancellation_token: Option<CancellationToken>,
}

impl<S: WebSocketSource> WebsocketService<S> {
    /// Creates a new `WebsocketService` with the provided WebSocketSource.
    pub fn new(socket: S) -> Self {
        Self {
            socket: Arc::new(Mutex::new(socket)),
            cached_prices: Arc::new(Mutex::new(HashMap::new())),
            cancellation_token: None,
        }
    }
}

#[async_trait::async_trait]
impl<S: WebSocketSource> Service for WebsocketService<S> {
    /// Starts the service, connecting to the WebSocket and subscribing to symbols.
    async fn start(&mut self, symbols: &[&str]) -> Result<(), Error> {
        if self.started() {
            return Err(Error::AlreadyStarted);
        }

        let mut locked_socket = self.socket.lock().await;
        if !locked_socket.is_connected() {
            locked_socket.connect().await?;
            locked_socket.subscribe(symbols).await?;
        }
        drop(locked_socket);

        let token = CancellationToken::new();
        let cloned_token = token.clone();
        let cloned_socket = Arc::clone(&self.socket);
        let cloned_cached_prices = Arc::clone(&self.cached_prices);
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
                            Some(Ok(WebsocketMessage::PriceInfo(price_info))) => {
                                let mut locked_cached_prices = cloned_cached_prices.lock().await;
                                locked_cached_prices.insert(price_info.symbol.to_string(), price_info);
                            }
                            Some(Ok(WebsocketMessage::SettingResponse(_response))) => {}
                            Some(Err(_)) => {}
                            None => {
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Stops the service, cancelling the WebSocket subscription.
    fn stop(&mut self) {
        if let Some(token) = &self.cancellation_token {
            token.cancel();
        }
        self.cancellation_token = None;
    }

    // To check if the service is started.
    fn started(&self) -> bool {
        self.cancellation_token.is_some()
    }
}

#[async_trait::async_trait]
impl<S: WebSocketSource> Source for WebsocketService<S> {
    /// Retrieves prices for the specified symbols from the cached prices.
    async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        let locked_cached_prices = self.cached_prices.lock().await;
        symbols
            .iter()
            .map(|&symbol| {
                locked_cached_prices
                    .get(&symbol.to_ascii_uppercase())
                    .map_or_else(
                        || Err(Error::NotFound(symbol.to_string())),
                        |price| Ok(price.clone()),
                    )
            })
            .collect()
    }

    // Asynchronous function to get price for a symbol.
    async fn get_price(&self, symbol: &str) -> Result<PriceInfo, Error> {
        self.get_prices(&[symbol])
            .await
            .pop()
            .ok_or(Error::Unknown)?
    }
}
