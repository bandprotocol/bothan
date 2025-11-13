//! Error types for Band REST API client operations.
//!
//! This module provides custom error types used throughout the Band REST API integration,
//! particularly for REST API client configuration and concurrent background data fetching.

use thiserror::Error;

/// Errors from initializing the Band REST API builder.
///
/// These errors can occur during the initialization and configuration of the HTTP client
/// or while constructing request parameters.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Indicates the provided URL was invalid.
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    /// Represents general failures during HTTP client construction (e.g., TLS configuration issues).
    #[error("reqwest error: {0}")]
    FailedToBuild(#[from] reqwest::Error),
}

/// Errors from fetching and handling data from the Band REST API.
///
/// These errors typically occur during API calls, response parsing, or data validation.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Indicates HTTP request failure due to network issues or HTTP errors.
    #[error("failed to fetch prices: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Indicates a failure to parse the API response.
    #[error("parse error: {0}")]
    ParseError(#[from] ParseError),
}

/// Errors that can occur while parsing Band API responses.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Indicates that the price value is not a valid number (NaN).
    #[error("price is NaN")]
    InvalidPrice,
    /// Indicates that the timestamp value is missing or invalid.
    #[error("invalid timestamp")]
    InvalidTimestamp,
}
