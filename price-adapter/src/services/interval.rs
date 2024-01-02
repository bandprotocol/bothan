use crate::types::{Service, Source};
use crate::{error::Error, types::PriceInfo};
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::time::sleep;
use tokio::{select, sync::Mutex};
use tokio_util::sync::CancellationToken;

/// A caching object storing prices received from Source at regular intervals.
pub struct IntervalService<S: Source> {
    adapter: Arc<Mutex<S>>,
    interval: Duration,
    cached_prices: Arc<Mutex<HashMap<String, PriceInfo>>>,
    cancellation_token: Option<CancellationToken>,
}

impl<S: Source> IntervalService<S> {
    /// Creates a new `IntervalService` with the provided Source.
    pub fn new(adapter: S, interval: Duration) -> Self {
        Self {
            adapter: Arc::new(Mutex::new(adapter)),
            interval,
            cached_prices: Arc::new(Mutex::new(HashMap::new())),
            cancellation_token: None,
        }
    }
}

#[async_trait::async_trait]
impl<S: Source> Service for IntervalService<S> {
    /// Starts the service, fetching prices at regular intervals and caching them.
    async fn start(&mut self, symbols: &[&str]) -> Result<(), Error> {
        if self.started() {
            return Err(Error::AlreadyStarted);
        }

        let token = CancellationToken::new();
        let cloned_token = token.clone();
        let cloned_adapter = Arc::clone(&self.adapter);
        let cloned_symbols: Vec<String> = symbols.iter().map(|&s| s.to_string()).collect();
        let cloned_cached_prices = Arc::clone(&self.cached_prices);
        let interval_duration = self.interval
        self.cancellation_token = Some(token);

        tokio::spawn(async move {
            loop {
                let borrowed_symbols: Vec<&str> =
                    cloned_symbols.iter().map(|s| s.as_str()).collect();
                let locked_adapter = cloned_adapter.lock().await;

                select! {
                    _ = cloned_token.cancelled() => {
                        break;
                    }

                    prices = locked_adapter.get_prices(&borrowed_symbols) => {
                        drop(locked_adapter);

                        let mut locked_cached_prices = cloned_cached_prices.lock().await;
                        for price in prices.into_iter().flatten() {
                            locked_cached_prices.insert(price.symbol.to_string(), price);
                        }
                    }
                }

                sleep(interval_duration).await;
            }
        });

        Ok(())
    }

    /// Stops the service, cancelling the interval fetching.
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
impl<S: Source> Source for IntervalService<S> {
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
