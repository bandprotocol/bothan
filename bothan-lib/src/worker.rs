//! Workers for asset data collection and information updates.
//!
//! This module defines traits and implementations for workers that monitor and update asset information.
//! Workers are responsible for collecting data from various sources and updating asset information in the store.
//!
//! The module provides:
//!
//! - The [`AssetWorker`] trait which defines the common interface for all asset workers
//! - Specialized worker implementations for different protocols and data sources
//! - Error handling specific to asset workers
//!
//! # Worker Types
//!
//! The module includes various worker implementations:
//!
//! - REST-based workers in the [`rest`] module for HTTP/HTTPS API polling
//! - WebSocket-based workers in the [`websocket`] module for real-time data streaming
//!
//! # Worker Lifecycle
//!
//! Workers follow a typical lifecycle pattern:
//!
//! 1. **Initialization**: Workers are built using their `build` method and configured with options
//! 3. **Running**: Workers run continuously, collecting data from sources and updating the store
//! 4. **Error Handling**: Workers should handle temporary failures gracefully with retries
//! 5. **Shutdown**: Workers should support graceful shutdown when requested
//!
//! # Worker Storage
//!
//! Workers should use the [`WorkerStore`](crate::store::WorkerStore) wrapper to interact with the main store.
//! This wrapper provides namespace isolation for each worker, ensuring that different workers
//! can operate without conflicts. Each worker should use the WorkerStore to update asset
//! information through the `set_asset_info` and `set_batch_asset_info` methods.
//!
//! # Extensibility
//!
//! While this module provides general-purpose implementations that cover common use cases,
//! the [`AssetWorker`] trait is designed for extensibility. Users can create custom worker 
//! implementations by implementing this trait to support specialized data sources, protocols,
//! or business logic beyond what's provided in the standard implementations.
//!
//! # Implementation
//!
//! When implementing a new worker type:
//!
//! 1. Implement the [`AssetWorker`] trait
//! 2. Define appropriate configuration options via the associated `Opts` type
//! 3. Implement the `build` method to create worker instances from configuration
//! 4. Implement any protocol-specific functionality in the worker
//! 5. Use [`WorkerStore`](crate::store::WorkerStore) to store asset information
//!
//! Workers are typically registered with a worker registry and managed by the system
//! to handle data collection and updates in a coordinated way.

use error::AssetWorkerError;

use crate::store::Store;

pub mod error;
pub mod rest;
pub mod websocket;

/// The universal trait for all workers that collect and update asset info.
///
/// This trait defines the common interface that all asset workers must implement,
/// allowing the system to interact with different types of workers in a uniform way.
/// Each worker is responsible for monitoring specific assets and updating their information
/// in the store.
///
/// Worker implementations should:
/// - Collect data from sources using appropriate methods
/// - Parse and validate the received data
/// - Update asset information using the [`WorkerStore`](crate::store::WorkerStore)
/// - Handle errors and implement retry mechanisms
/// - Support graceful shutdown
///
/// # Type Parameters
///
/// * `Opts` - Configuration options for the worker
///
/// # Required Methods
///
/// Implementors must define:
/// * `name()` - Returns a unique identifier for the worker type
/// * `build()` - Creates a new worker instance with the specified options and query IDs
///
/// # Worker Store Usage
///
/// Workers should use the [`WorkerStore`](crate::store::WorkerStore) to interact with 
/// the main store. This provides namespace isolation and ensures that different workers 
/// can operate without conflicts. Typically, a worker implementation will:
///
/// 1. Receive a reference to the main store in its `build` method
/// 2. Create a `WorkerStore` instance with the worker's name as prefix
/// 3. Use the `WorkerStore` to update asset information
///
/// # Examples
///
/// Implementing a simple HTTP polling worker:
///
/// ```rust
/// use std::time::Duration;
/// use async_trait::async_trait;
/// use bothan_lib::worker::{AssetWorker, error::AssetWorkerError};
/// use bothan_lib::store::{Store, WorkerStore};
/// use bothan_lib::types::AssetInfo;
/// use rust_decimal::Decimal;
///
/// // Worker configuration
/// #[derive(Clone)]
/// struct HttpPollerOpts {
///     poll_interval: Duration,
///     endpoint: String,
/// }
///
/// // Worker implementation
/// struct HttpPoller {
///     opts: HttpPollerOpts,
///     ids: Vec<String>,
///     store: WorkerStore<Box<dyn Store>>,
/// }
///
/// #[async_trait]
/// impl AssetWorker for HttpPoller {
///     type Opts = HttpPollerOpts;
///
///     fn name(&self) -> &'static str {
///         "http_poller"
///     }
///
///     async fn build<S: Store + 'static>(
///         opts: Self::Opts,
///         store: &S,
///         ids: Vec<String>,
///     ) -> Result<Self, AssetWorkerError> {
///         // Create a worker-specific store with the worker's name as prefix
///         let worker_store = WorkerStore::new(store, "http_poller");
/// 
///         // In a real implementation, you would also have methods for:
///         // - Starting the worker (data collection loop)
///         // - Handling errors and retries
///         // - Shutting down gracefully
///         
///         Ok(HttpPoller { 
///             opts, 
///             ids,
///             store: worker_store,
///         })
///     }
///     
/// }
///
/// // Example of how the worker might update asset information
/// async fn update_asset_example(worker: &HttpPoller) -> Result<(), Box<dyn std::error::Error>> {
///     // Fetch data from the endpoint (simplified)
///     let asset_data = fetch_from_api(&worker.opts.endpoint, &worker.ids[0]).await?;
///     
///     // Create AssetInfo from the data
///     let asset_info = AssetInfo::new(
///         worker.ids[0].clone(),
///         Decimal::new(asset_data.price * 100, 2), // Convert to Decimal
///         asset_data.timestamp,
///     );
///     
///     // Store the updated asset info
///     worker.store.set_asset_info(asset_info).await?;
///     
///     Ok(())
/// }
///
/// // Simplified API fetching function
/// async fn fetch_from_api(endpoint: &str, id: &str) -> Result<ApiResponse, reqwest::Error> {
///     // In a real implementation, this would make an HTTP request
///     Ok(ApiResponse {
///         price: 395.0,
///         timestamp: 1634567890000,
///     })
/// }
///
/// struct ApiResponse {
///     price: f64,
///     timestamp: i64,
/// }
/// ```
#[async_trait::async_trait]
pub trait AssetWorker: Send + Sync + Sized {
    /// Configuration options for the worker.
    ///
    /// This associated type defines the configuration parameters needed to
    /// construct an instance of the worker.
    type Opts;

    /// Returns the name of the worker.
    ///
    /// The name serves as a unique identifier for the worker. When used in conjunction
    /// with Bothan's registry, the worker's name should match the source name specified 
    /// in the registry.
    fn name(&self) -> &'static str;

    /// Builds a new worker instance with the specified options and query IDs.
    ///
    /// This method is responsible for creating and initializing a new worker
    /// instance with the provided configuration and query IDs to monitor.
    ///
    /// # Errors
    ///
    /// Returns an `AssetWorkerError` if the worker cannot be built with the given
    /// parameters, such as when the configuration is invalid or required resources
    /// are unavailable.
    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError>;
}
