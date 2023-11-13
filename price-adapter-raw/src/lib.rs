//! # price-explorer
//! price-explorer is a price query adapter for crypto exchanges and price aggregators.
//!
//! It provides a unified interface for querying price data from different exchanges and
//! price aggregators. Currently, it supports the following exchanges and price aggregators:
//! - Binance
//! - CoinGecko

//! # Usage
//!
//! ## Coingecko
//! To use coingecko api, you need to create a `CoingeckoPro` instance and set the api key.
//! ```rust
//! use price_adapter_raw::CoingeckoPro;
//!
//! #[tokio::main]
//! async fn main() {
//!     let coingecko = CoingeckoPro::new().set_api_key("$API_KEY".into());
//!     let queries = vec![("ethereum", "USD")];
//!     let prices = coingecko.get_prices(&queries).await;
//!     println!("prices: {:?}", prices);
//! }
//! ````
//!
//! ## Binance Websocket
//! To use binance websocket api, you need to create a `BinanceWebsocket` instance and set the
//! query symbols.
//! ```rust
//! use price_adapter_raw::BinanceWebsocketBuilder;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!    let mut binance_ws = BinanceWebsocketBuilder::new()
//!         .set_query_symbols(&[("eth", "btc"), ("btc", "usdt")])
//!         .build()
//!         .await
//!         .unwrap();
//!     while let Some(data) = binance_ws.next().await {
//!         match data {
//!             Ok(price) => {
//!                 println!("price: {}", price);
//!                 # break;
//!             }
//!             Err(e) => {
//!                 eprintln!("Error: {}", e);
//!                 break;
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Or use `BinanceWebsocketService` to query price data.
//! ```rust
//! use price_adapter_raw::{BinanceWebsocketBuilder, BinanceWebsocketService};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!    let mut binance_ws = BinanceWebsocketBuilder::new()
//!         .set_query_symbols(&[("eth", "btc"), ("btc", "usdt")])
//!         .build()
//!         .await
//!         .unwrap();
//!     let mut service = BinanceWebsocketService::new(binance_ws);
//!     service.start().unwrap();
//!     tokio::time::sleep(Duration::from_secs(1)).await;
//!
//!     let price = service.get_price("btc", "usdt").await;
//!     println!("price: {:?}", price);
//! }
//! ```

mod binance_websocket;
mod coingecko;
mod error;
mod types;

pub use binance_websocket::{BinanceWebsocket, BinanceWebsocketBuilder, BinanceWebsocketService};
pub use coingecko::{CoingeckoPro, CoingeckoPublic};
