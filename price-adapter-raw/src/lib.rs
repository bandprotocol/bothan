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
//! use price_adapter_raw::CoinGecko;
//!
//! #[tokio::main]
//! async fn main() {
//!     let coingecko = CoinGecko::new_with_api_key("$API_KEY".into());
//!     let queries = vec!["ethereum"];
//!     let prices = coingecko.get_prices(&queries).await;
//!     println!("prices: {:?}", prices);
//! }
//! ````
//!
//! ## Binance Websocket
//! To use binance websocket api, you need to create a `BinanceWebsocket` instance and set the
//! query symbols.
//! ```rust
//! use price_adapter_raw::BinanceWebsocket;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut binance_ws = BinanceWebsocket::new("wss://stream.binance.com:9443");
//!     binance_ws.connect().await.unwrap();
//!     binance_ws.subscribe(&["ethbtc", "btcusdt"]).await;
//!     let data = binance_ws.next().await.unwrap();
//!     match data {
//!         Ok(price) => {
//!             println!("price: {:?}", price);
//!         }
//!         Err(e) => {
//!             eprintln!("Error: {}", e);
//!         }
//!     }
//! }
//! ```
//!
//! Or use `BinanceWebsocketService` to query price data.
//! ```rust
//! use price_adapter_raw::{BinanceWebsocket, BinanceWebsocketService};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut binance_ws = BinanceWebsocket::new("wss://stream.binance.com:9443");
//!     let mut service = BinanceWebsocketService::new(binance_ws);
//!     service.start(&["ethbtc", "btcusdt"]).await.unwrap();
//!     tokio::time::sleep(Duration::from_secs(1)).await;
//!
//!     let price = service.get_prices(&["btcusdt"]).await;
//!     println!("price: {:?}", price);
//! }
//! ```

mod binance_websocket;
mod coingecko;
pub mod error;
pub mod types;

pub use binance_websocket::{BinanceWebsocket, BinanceWebsocketService};
pub use coingecko::CoinGecko;
