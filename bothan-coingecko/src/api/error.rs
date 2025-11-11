//! Error types for CoinGecko REST API client operations.
//!
//! This module provides custom error types used throughout the CoinGecko REST API integration,
//! particularly for REST API client configuration and concurrent background data fetching.

use reqwest::StatusCode;
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
    #[error("failed to fetch {resource}: {error}")]
    SendingRequestError {
        #[source]
        error: reqwest::Error,
        resource: String,
    },

    /// Indicates a non-success HTTP status code.
    #[error("returned HTTP {status} for {resource}: {body}")]
    HttpStatusError {
        status: StatusCode,
        body: String,
        resource: String,
    },

    /// Indicates the response body could not be parsed into the expected shape.
    #[error("failed to parse {resource}: {source}")]
    ParseResponseError {
        #[source]
        source: reqwest::Error,
        resource: String,
    },

    /// Indicates that the response data contains invalid numeric values (e.g., `NaN`).
    #[error("invalid price value {price} for id {id}")]
    InvalidValue { price: f64, id: String },
}
