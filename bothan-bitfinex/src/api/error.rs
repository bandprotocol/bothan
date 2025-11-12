//! Error types for Bitfinex API client operations.
//!
//! This module provides custom error types used throughout the Bitfinex API integration,
//! particularly for handling REST API requests and price validation errors.
use reqwest::StatusCode;
use thiserror::Error;

/// Errors related to Bitfinex API client configuration and building.
///
/// These errors can occur during the construction of the Bitfinex REST API client,
/// such as invalid URLs or HTTP client creation failures.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Indicates that the provided URL is invalid and cannot be parsed.
    #[error("invalid url")]
    InvalidURL(#[from] url::ParseError),

    /// Indicates a failure to build the HTTP client for the Bitfinex API.
    #[error("failed to build with error: {0}")]
    FailedToBuild(#[from] reqwest::Error),
}

/// Errors encountered while fetching data from the Bitfinex API.
///
/// These errors can occur during REST API requests to the Bitfinex API,
/// such as network failures, invalid responses, or data parsing issues.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Indicates a failure to fetch ticker data from the Bitfinex API.
    #[error("failed to fetch tickers (symbols={symbols}): {error}")]
    SendingRequestError {
        #[source]
        error: reqwest::Error,
        symbols: String,
    },

    /// Indicates a non-success HTTP status code.
    #[error("returned HTTP {status} for symbols={symbols}: {body}")]
    HttpStatusError {
        status: StatusCode,
        body: String,
        symbols: String,
    },

    /// Indicates the response body could not be parsed into the expected shape.
    #[error("failed to parse response for symbols={symbols}: {source}")]
    ParseResponseError {
        #[source]
        source: reqwest::Error,
        symbols: String,
    },
}
