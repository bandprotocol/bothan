//! Metrics instrumentation for Bothan services.
//!
//! This module provides metrics collection and observability utilities for various Bothan components.
//! It exposes submodules for REST polling, gRPC server, store operations, websocket connections, and utility helpers.
//!
//! - [`rest`]: Metrics for REST polling operations
//! - [`server`]: Metrics for gRPC server requests
//! - [`store`]: Metrics for store operations
//! - [`websocket`]: Metrics for websocket connections and messages
//! - [`utils`]: Utility helpers for metrics labeling and conversion

pub mod rest;
pub mod server;
pub mod store;
pub mod utils;
pub mod websocket;
