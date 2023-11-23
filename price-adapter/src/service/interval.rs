use crate::types::PriceAdapter;
use crate::{error::Error, types::PriceInfo};
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::time::sleep;
use tokio::{select, sync::Mutex};
use tokio_util::sync::CancellationToken;

/// A caching object storing prices received from binance websocket.
pub struct IntervalService<P: PriceAdapter> {
    adapter: Arc<Mutex<P>>,
    cached_price: Arc<Mutex<HashMap<String, PriceInfo>>>,
    cancellation_token: Option<CancellationToken>,
}

impl<P: PriceAdapter> IntervalService<P> {
    /// initiate new object from created socket.
    pub fn new(adapter: P) -> Self {
        Self {
            adapter: Arc::new(Mutex::new(adapter)),
            cached_price: Arc::new(Mutex::new(HashMap::new())),
            cancellation_token: None,
        }
    }

    /// start a service.
    pub async fn start(&mut self, symbols: &[&str], interval_sec: u64) -> Result<(), Error> {
        if self.cancellation_token.is_some() {
            return Err(Error::AlreadyStarted);
        }

        let token = CancellationToken::new();
        let cloned_token = token.clone();
        let cloned_adapter = Arc::clone(&self.adapter);
        let cloned_symbols: Vec<String> = symbols.iter().map(|&s| s.to_string()).collect();
        let cloned_cached_price = Arc::clone(&self.cached_price);
        let interval_duration = Duration::from_secs(interval_sec);
        self.cancellation_token = Some(token);

        tokio::spawn(async move {
            loop {
                let borrowed_symbols: Vec<&str> =
                    cloned_symbols.iter().map(|s| s.as_str()).collect();
                let locked_adapter: tokio::sync::MutexGuard<'_, P> = cloned_adapter.lock().await;
                select! {
                    _ = cloned_token.cancelled() => {
                        break;
                    }

                    prices = locked_adapter.get_prices(borrowed_symbols.as_slice()) => {
                        drop(locked_adapter);

                        for price in prices.into_iter().flatten() {
                            let mut locked_cached_price = cloned_cached_price.lock().await;
                            locked_cached_price.insert(price.symbol.to_string(), price);
                        }
                    }
                }

                sleep(interval_duration).await;
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

    pub async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>> {
        let mut prices = Vec::new();
        let locked_cached_price = self.cached_price.lock().await;

        for &symbol in symbols {
            let price = match locked_cached_price.get(&symbol.to_ascii_uppercase()) {
                Some(price) => Ok(price.clone()),
                None => Err(Error::NotFound(symbol.to_string())),
            };

            prices.push(price);
        }

        prices
    }
}
