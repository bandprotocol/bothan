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

    /// Indicates an HTTP header value was invalid or contained prohibited characters.
    #[error("invalid header value")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// Represents general failures during HTTP client construction (e.g., TLS configuration issues).
    #[error("reqwest error: {0}")]
    FailedToBuild(#[from] reqwest::Error),
}

/// General errors from Band API operations.
///
/// These errors typically occur during API calls, response parsing, or data validation.
#[derive(Debug, Error)]
pub enum Error {
    /// Indicates the requested limit is too high (must be <= 5000).
    #[error("limit must be lower or equal to 5000")]
    LimitTooHigh,

    /// Indicates an HTTP request failure due to network issues or HTTP errors.
    #[error("failed request: {0}")]
    FailedRequest(#[from] reqwest::Error),
}

/// Errors from fetching and handling data from the Band REST API.
///
/// These errors typically occur during API calls, response parsing, or data validation.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Indicates that an ID in the request is not a valid integer.
    #[error("ids contains non integer value")]
    InvalidId,

    /// Indicates HTTP request failure due to network issues or HTTP errors.
    #[error("failed to fetch tickers: {0}")]
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
}
