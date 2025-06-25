//! # Bothan API Server Library
//!
//! This library provides the core server implementation for the Bothan API,
//! which serves as a centralized gateway for cryptocurrency data aggregation
//! and processing.
//!
//! ## Overview
//!
//! The Bothan API Server is responsible for:
//! - Exposing gRPC and REST endpoints for data access
//! - Managing connections to various cryptocurrency exchanges
//! - Processing and aggregating market data
//! - Providing real-time price feeds and market information
//!
//! ## Key Components
//!
//! - **API Module**: gRPC and REST server implementations
//! - **Config Module**: Configuration management and validation
//! - **Proto Module**: Protocol buffer definitions and generated code
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::{api, config};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = config::AppConfig::with_name("config")?;
//!     // Initialize and run server
//!     Ok(())
//! }
//! ```
//!
//! ## Version Information
//!
//! The library version and registry requirements are exposed as constants
//! for integration with the broader Bothan ecosystem.

pub mod api;
pub mod config;
pub mod proto;

/// Current version of the Bothan API Server library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Required registry version for compatibility
pub const REGISTRY_REQUIREMENT: &str = "^0.0";
