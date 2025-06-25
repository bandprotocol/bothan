//! # API Module
//!
//! This module provides the core API server implementation for the Bothan API,
//! including gRPC and REST endpoints for cryptocurrency data access.
//!
//! ## Components
//!
//! - **BothanServer**: Main server implementation handling gRPC and REST requests
//! - **Server Module**: Internal server implementation details
//! - **Utils Module**: Utility functions for API operations
//!
//! ## Features
//!
//! - gRPC server with protocol buffer support
//! - REST API endpoints with JSON responses
//! - Real-time data streaming capabilities
//! - Authentication and authorization
//! - Rate limiting and request validation
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::api::BothanServer;
//! use bothan_api::config::AppConfig;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = AppConfig::default();
//!     // Initialize server with config
//!     Ok(())
//! }
//! ```

pub use server::BothanServer;

mod server;
mod utils;
