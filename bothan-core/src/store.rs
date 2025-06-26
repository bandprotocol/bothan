//! Store abstraction and implementations for Bothan core.
//!
//! Provides storage backends (e.g., RocksDB).

#[cfg(feature = "rocksdb")]
pub mod rocksdb;
