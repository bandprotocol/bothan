//! Error types for CoinGecko REST API client operations.
//!
//! This module provides custom error types used throughout the CoinGecko REST API integration,
//! particularly for REST API client configuration and concurrent background data fetching.

use thiserror::Error;

/// Errors from initializing the CoinGecko REST API builder.
///
/// These errors can occur during the initialization and configuration of the HTTP client
/// or while constructing request parameters.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Indicates the provided URL was invalid.
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    /// Indicates an HTTP header value was invalid or contained prohibited characters.
    #[error("invalid header value")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// Represents general failures during HTTP client construction (e.g., TLS configuration issues).
    #[error("failed to build with error: {0}")]
    FailedToBuild(#[from] reqwest::Error),
}

/// Errors from fetching and handling data from the CoinGecko REST API.
///
/// These errors typically occur during API calls, response parsing, or data validation.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Indicates HTTP request failure due to network issues or HTTP errors.
    #[error("failed to fetch tickers: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Indicates that the response data contains invalid numeric values (e.g., `NaN`).
    #[error("value contains nan")]
    InvalidValue,
}
