//! # Client Module
//!
//! This module provides client implementations for interacting with the Bothan API Server,
//! supporting both gRPC and REST protocols.
//!
//! ## Components
//!
//! - **GrpcClient**: High-performance gRPC client for binary protocol communication
//! - **RestClient**: HTTP-based REST client for JSON API interactions
//!
//! ## Protocol Support
//!
//! ### gRPC Client
//!
//! The gRPC client provides:
//! - Binary protocol for maximum performance
//! - Streaming support for real-time data
//! - Strong typing with protocol buffers
//! - Connection pooling and load balancing
//!
//! ### REST Client
//!
//! The REST client provides:
//! - HTTP/JSON interface for easy integration
//! - Standard REST conventions
//! - Browser-friendly API access
//! - Simple authentication and headers
//!
//! ## Usage Examples
//!
//! ```rust,no_run
//! use bothan_client::client::{GrpcClient, RestClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // gRPC client
//!     let grpc_client = GrpcClient::new("http://localhost:9090").await?;
//!     
//!     // REST client
//!     let rest_client = RestClient::new("http://localhost:8080")?;
//!     
//!     Ok(())
//! }
//! ```

#![allow(unused_imports)]
#![allow(dead_code)]

/// gRPC client for Bothan API Server communication
pub use grpc::GrpcClient;

/// REST client for Bothan API Server communication
pub use rest::RestClient;

mod grpc;
mod rest;
