//! # Bothan API Rust Client Library
//!
//! This library provides a Rust client for interacting with the Bothan API Server,
//! offering both gRPC and REST interfaces for cryptocurrency data access.
//!
//! ## Overview
//!
//! The Bothan API Rust Client provides:
//! - High-level client abstractions for API interactions
//! - Protocol buffer definitions and generated code
//! - Support for both gRPC and REST endpoints
//! - Async/await support for non-blocking operations
//! - Comprehensive error handling and type safety
//!
//! ## Features
//!
//! - **gRPC Client**: High-performance binary protocol client
//! - **REST Client**: HTTP-based REST API client
//! - **Protocol Buffers**: Type-safe message definitions
//! - **Async Support**: Built on tokio for efficient async operations
//! - **Error Handling**: Comprehensive error types and handling
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_client::{client::BothanClient, proto::bothan::v1::PriceRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = BothanClient::new("http://localhost:9090").await?;
//!     
//!     let request = PriceRequest {
//!         symbol: "BTC/USD".to_string(),
//!         ..Default::default()
//!     };
//!     
//!     let response = client.get_price(request).await?;
//!     println!("Price: {}", response.price);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Modules
//!
//! - **client**: Main client implementations and abstractions
//! - **proto**: Protocol buffer definitions and generated code

pub mod client;
pub mod proto;
