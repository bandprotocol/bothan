//! Common type definitions used throughout the library.
//!
//! This module serves as a central location for core data structures and types
//! that are used across different components of the system. It re-exports these
//! types to provide a clean and consistent interface for other modules.
//!
//! The module includes:
//!
//! - [`AssetInfo`] - Core data structure for representing asset price and timestamp information
//!
//! By centralizing common types in this module, we ensure consistency throughout
//! the codebase and provide a single point of reference for understanding the
//! system's fundamental data structures.

pub use asset_info::AssetInfo;

mod asset_info;
