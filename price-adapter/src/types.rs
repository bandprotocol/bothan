use futures_util::{stream::FusedStream, Stream, StreamExt};
use serde::Deserialize;
use serde_json::Value;

use crate::error::Error;
use core::fmt;

#[derive(Clone, Debug)]
pub struct PriceInfo {
    pub symbol: String,
    pub price: f64,
    pub timestamp: u64,
}

impl fmt::Display for PriceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PriceInfo {{ symbol: {}, price: {}, timestamp: {} }}",
            self.symbol, self.price, self.timestamp
        )
    }
}

#[async_trait::async_trait]
pub trait PriceAdapter: Send + Sync + 'static {
    async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>>;
}

#[async_trait::async_trait]
pub trait WebsocketPriceAdapter: Send + Sync + StreamExt + FusedStream + Unpin + 'static {
    async fn connect(&mut self) -> Result<(), Error>;
    async fn subscribe(&mut self, symbols: &[&str]) -> Result<u32, Error>;
    async fn unsubscribe(&mut self, symbols: &[&str]) -> Result<u32, Error>;
    fn is_connected(&self) -> bool;
}

// #[async_trait::async_trait]
// pub trait ServicePriceAdapter: Send + Sync + 'static {
//     async fn start(&mut self, symbols: &[&str]) -> Result<(), Error>;
//     fn stop(&mut self);
//     async fn get_prices(&self, symbols: &[&str]) -> Vec<Result<PriceInfo, Error>>;
// }

#[derive(Debug, Deserialize)]
pub struct SettingResponse {
    pub data: Value,
}

#[derive(Debug)]
pub enum WebsocketMessage {
    PriceInfo(PriceInfo),
    SettingResponse(SettingResponse),
}
