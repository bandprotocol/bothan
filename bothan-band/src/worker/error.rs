//! Error types for CoinMarketCap worker operations.
//!
//! This module provides custom error types used throughout the CoinMarketCap worker integration,
//! particularly for asset polling and data fetching.

use thiserror::Error;

/// Errors from fetching and handling data in the CoinMarketCap worker.
///
/// These errors typically occur during API calls, response parsing, or data validation in the worker context.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Indicates that the response data contains invalid numeric values (e.g., `NaN`).
    #[error("value contains nan")]
    InvalidValue,
}
