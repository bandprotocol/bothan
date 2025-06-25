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
    /// Indicates that an ID in the request is not a valid integer.
    #[error("ids contains non integer value: {0}")]
    InvalidId(String),

    /// Indicates HTTP request failure due to network issues or HTTP errors.
    #[error("failed to fetch tickers: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Indicates that a required value was missing in the response.
    #[error("missing value")]
    MissingValue,

    /// Indicates that the response data contains invalid numeric values (e.g., `NaN`).
    #[error("value contains nan")]
    InvalidValue,
}
