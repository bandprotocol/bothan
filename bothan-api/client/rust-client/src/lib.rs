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
//! ## Modules
//!
//! - **client**: Main client implementations and abstractions
//! - **proto**: Protocol buffer definitions and generated code

pub mod client;
pub mod proto;
