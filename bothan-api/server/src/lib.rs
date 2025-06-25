//! Bothan API server main module.
//!
//! Core server implementation for cryptocurrency data aggregation and processing.

pub mod api;
pub mod config;
pub mod proto;

/// Current version of the Bothan API Server library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Required registry version for compatibility
pub const REGISTRY_REQUIREMENT: &str = "^0.0";
