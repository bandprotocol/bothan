//! # gRPC Configuration Module
//!
//! This module provides configuration settings for the gRPC server component
//! of the Bothan API Server.
//!
//! ## Overview
//!
//! The gRPC server configuration handles:
//! - Server binding address and port
//! - Network interface selection
//! - Default endpoint configuration
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bothan_api::config::grpc::GrpcConfig;
//! use std::net::SocketAddr;
//!
//! // Use default configuration
//! let config = GrpcConfig::default();
//!
//! // Custom configuration
//! let config = GrpcConfig {
//!     addr: SocketAddr::from(([127, 0, 0, 1], 9090)),
//! };
//! ```
//!
//! ## Configuration Example
//!
//! ```toml
//! [grpc]
//! addr = "0.0.0.0:50051"
//! ```

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

/// Configuration for the Bothan API Server's gRPC server.
///
///
/// This struct defines the network configuration for the gRPC server,
/// including the address and port where the server will listen for
/// incoming connections.
///
/// ## Default Configuration
///
/// By default, the gRPC server binds to `0.0.0.0:50051`, which means
/// it will accept connections on all network interfaces on port 50051.
///
/// ## Example Configuration
///
/// ```toml
/// [grpc]
/// # Bind to all interfaces on port 9090
/// addr = "0.0.0.0:9090"
///
/// # Or bind to specific interface
/// addr = "127.0.0.1:50051"
/// ```
///
/// ## Security Considerations
///
/// When deploying to production, consider:
/// - Binding to specific network interfaces rather than `0.0.0.0`
/// - Using non-standard ports for security through obscurity
/// - Implementing proper firewall rules
/// - Using TLS for encrypted communication
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrpcConfig {
    /// The network address and port to bind the gRPC server to.
    ///
    /// This field specifies where the gRPC server will listen for
    /// incoming connections. The default value is `0.0.0.0:50051`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use bothan_api::config::grpc::GrpcConfig;
    /// use std::net::SocketAddr;
    ///
    /// let config = GrpcConfig {
    ///     addr: SocketAddr::from(([127, 0, 0, 1], 9090)),
    /// };
    /// ```
    #[serde(default = "default_addr")]
    pub addr: SocketAddr,
}

/// Returns the default gRPC server address.
///
/// The default address is `0.0.0.0:50051`, which binds the server
/// to all available network interfaces on port 50051.
///
/// # Returns
///
/// Returns a `SocketAddr` representing the default gRPC server binding address.
///
/// # Example
///
/// ```rust,no_run
/// use bothan_api::config::grpc::GrpcConfig;
///
/// let config = GrpcConfig::default();
/// println!("Default gRPC address: {}", config.addr);
/// // Output: Default gRPC address: 0.0.0.0:50051
/// ```
pub fn default_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 50051))
}

impl Default for GrpcConfig {
    /// Creates a new `GrpcConfig` with default values.
    ///
    /// The default configuration binds the gRPC server to `0.0.0.0:50051`.
    ///
    /// # Returns
    ///
    /// Returns a `GrpcConfig` instance with default network settings.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use bothan_api::config::grpc::GrpcConfig;
    ///
    /// let config = GrpcConfig::default();
    /// assert_eq!(config.addr.to_string(), "0.0.0.0:50051");
    /// ```
    fn default() -> Self {
        GrpcConfig {
            addr: default_addr(),
        }
    }
}
