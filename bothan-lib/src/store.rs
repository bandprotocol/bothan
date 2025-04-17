//! Storage abstractions for asset information and registry data.
//!
//! This module provides traits and implementations for persistent storage of asset information
//! and registry data. It defines a common interface for different storage backends while
//! ensuring thread safety and atomic operations.
//!
//! The module provides:
//!
//! - The [`Store`] trait which defines the common interface for all storage implementations
//! - Specialized store implementations for different storage backends
//! - Worker-specific store functionality through the [`WorkerStore`] trait
//!
//! # Storage Capabilities
//!
//! The store is responsible for:
//!
//! - Storing and retrieving the registry and its IPFS hash
//! - Managing asset information with prefix-based organization
//! - Supporting both individual and batch operations for asset data
//!
//! # Extensibility
//!
//! While this module provides general-purpose implementations for common storage backends,
//! the [`Store`] trait is designed for extensibility. Users can create custom storage
//! implementations by implementing this trait to support specialized backends, caching
//! strategies, or data transformation logic beyond what's provided in the standard implementations.
//!
//! # Implementation
//!
//! When implementing a custom store:
//!
//! 1. Implement the [`Store`] trait
//! 2. Define an appropriate `Error` type that captures all possible storage failures
//! 3. Ensure thread safety and atomicity in all operations
//! 4. Make the implementation efficiently cloneable for use across tasks

use std::error::Error as StdError;

use async_trait::async_trait;
pub use worker::WorkerStore;

use crate::registry::{Registry, Valid};
use crate::types::AssetInfo;

mod worker;

/// The universal trait for all stores. All implementations must be thread-safe and atomic.
///
/// This trait defines the common interface that all storage implementations must provide,
/// enabling the system to interact with different storage backends in a uniform way.
/// Each store is responsible for reliably persisting asset information and registry data.
///
/// # Thread Safety and Atomicity
///
/// All implementations must be thread-safe (Send + Sync) and provide atomic operations
/// to ensure data consistency when used concurrently.
///
/// # Type Parameters
///
/// * `Error` - A custom error type that captures all storage-specific failure conditions
///
/// # Required Methods
///
/// Implementors must define:
/// * Storage operations for registry data (`set_registry`, `get_registry`, `get_registry_ipfs_hash`)
/// * Asset information operations (`get_asset_info`, `insert_asset_info`, `insert_batch_asset_info`)
///
/// # Example Use
///
/// Implementing a simple in-memory store:
///
/// ```rust
/// use std::collections::HashMap;
/// use std::error::Error;
/// use std::fmt;
/// use std::sync::{Arc, RwLock};
/// use async_trait::async_trait;
/// use bothan_lib::store::Store;
/// use bothan_lib::registry::{Registry, Valid};
/// use bothan_lib::types::AssetInfo;
/// use thiserror::Error;
///
/// // Custom error type for the memory store
/// #[derive(Debug, Error)]
/// #[error("Memory store error: {0}")]
/// struct MemoryStoreError(String);
///
/// // Memory-based store implementation
/// #[derive(Clone)]
/// struct MemoryStore {
///     registry: Arc<RwLock<Option<(Registry<Valid>, String)>>>,
///     assets: Arc<RwLock<HashMap<String, HashMap<String, AssetInfo>>>>,
/// }
///
/// impl MemoryStore {
///     fn new() -> Self {
///         MemoryStore {
///             registry: Arc::new(RwLock::new(None)),
///             assets: Arc::new(RwLock::new(HashMap::new())),
///         }
///     }
/// }
///
/// #[async_trait]
/// impl Store for MemoryStore {
///     type Error = MemoryStoreError;
///
///     async fn set_registry(
///         &self,
///         registry: Registry<Valid>,
///         ipfs_hash: String,
///     ) -> Result<(), Self::Error> {
///         let mut reg = self.registry.write().map_err(|e| MemoryStoreError(e.to_string()))?;
///         *reg = Some((registry, ipfs_hash));
///         Ok(())
///     }
///
///     async fn get_registry(&self) -> Result<Registry<Valid>, Self::Error> {
///         let reg = self.registry.read().map_err(|e| MemoryStoreError(e.to_string()))?;
///         match &*reg {
///             Some((registry, _)) => Ok(registry.clone()),
///             None => Err(MemoryStoreError("Registry not found".to_string())),
///         }
///     }
///
///     async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error> {
///         let reg = self.registry.read().map_err(|e| MemoryStoreError(e.to_string()))?;
///         Ok(reg.as_ref().map(|(_, hash)| hash.clone()))
///     }
///
///     async fn get_asset_info(
///         &self,
///         prefix: &str,
///         id: &str,
///     ) -> Result<Option<AssetInfo>, Self::Error> {
///         let assets = self.assets.read().map_err(|e| MemoryStoreError(e.to_string()))?;
///         Ok(assets
///             .get(prefix)
///             .and_then(|prefix_map| prefix_map.get(id).cloned()))
///     }
///
///     async fn insert_asset_info(
///         &self,
///         prefix: &str,
///         asset_info: AssetInfo,
///     ) -> Result<(), Self::Error> {
///         let mut assets = self.assets.write().map_err(|e| MemoryStoreError(e.to_string()))?;
///         let prefix_map = assets.entry(prefix.to_string()).or_insert_with(HashMap::new);
///         prefix_map.insert(asset_info.id.clone(), asset_info);
///         Ok(())
///     }
///
///     async fn insert_batch_asset_info(
///         &self,
///         prefix: &str,
///         asset_infos: Vec<AssetInfo>,
///     ) -> Result<(), Self::Error> {
///         let mut assets = self.assets.write().map_err(|e| MemoryStoreError(e.to_string()))?;
///         let prefix_map = assets.entry(prefix.to_string()).or_insert_with(HashMap::new);
///         
///         for asset_info in asset_infos {
///             prefix_map.insert(asset_info.id.clone(), asset_info);
///         }
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Store: Send + Sync + Clone {
    /// The type returned in the event of an operation failure.
    ///
    /// This should be a custom error type that implements the standard Error trait 
    /// and captures all possible error conditions specific to the storage backend.
    type Error: StdError + Send + Sync + 'static;

    /// Stores a validated registry and its IPFS hash in the storage backend.
    ///
    /// This operation should be atomic to ensure registry consistency.
    /// It should replace any existing registry completely.
    ///
    /// # Errors
    ///
    /// Returns a storage-specific error if the operation fails, such as when
    /// the storage backend is unavailable or the data cannot be written.
    async fn set_registry(
        &self,
        registry: Registry<Valid>,
        ipfs_hash: String,
    ) -> Result<(), Self::Error>;

    /// Retrieves the current validated registry from the storage backend.
    /// 
    /// # Errors
    ///
    /// Returns a storage-specific error if the operation fails, such as when
    /// the storage backend is unavailable or the registry does not exist.
    async fn get_registry(&self) -> Result<Registry<Valid>, Self::Error>;

    /// Retrieves the IPFS hash of the current registry from the storage backend.
    /// 
    /// # Errors
    ///
    /// Returns a storage-specific error if the operation fails, such as when
    /// the storage backend is unavailable.
    async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error>;

    /// Retrieves asset information for a specific asset ID within a namespace prefix.
    ///
    /// # Errors
    ///
    /// Returns a storage-specific error if the operation fails, such as when
    /// the storage backend is unavailable.
    async fn get_asset_info(
        &self,
        prefix: &str,
        id: &str,
    ) -> Result<Option<AssetInfo>, Self::Error>;

    /// Stores asset information under the specified namespace prefix.
    ///
    /// If asset information already exists for the given prefix and asset ID,
    /// it should be completely replaced with the new information.
    ///
    /// # Errors
    ///
    /// Returns a storage-specific error if the operation fails, such as when
    /// the storage backend is unavailable or the data cannot be written.
    async fn insert_asset_info(
        &self,
        prefix: &str,
        asset_info: AssetInfo,
    ) -> Result<(), Self::Error>;

    /// Stores multiple asset information entries in a single batch operation.
    ///
    /// This operation should be optimized for bulk insertion and preferably
    /// performed atomically if the backend supports transactions.
    ///
    /// # Errors
    ///
    /// Returns a storage-specific error if the operation fails, such as when
    /// the storage backend is unavailable, the data cannot be written, or
    /// the transaction cannot be committed.
    async fn insert_batch_asset_info(
        &self,
        prefix: &str,
        asset_infos: Vec<AssetInfo>,
    ) -> Result<(), Self::Error>;
}
