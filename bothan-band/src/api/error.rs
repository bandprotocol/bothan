//! Error types for Band REST API client operations.
//!
//! This module provides custom error types used throughout the Band REST API integration,
//! particularly for REST API client configuration and concurrent background data fetching.

use reqwest::StatusCode;
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
    #[error("failed to build with error: {0}")]
    FailedToBuild(#[from] reqwest::Error),
}

/// Errors from fetching and handling data from the Band REST API.
///
/// These errors typically occur during API calls, response parsing, or data validation.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Indicates HTTP request failure due to network issues or HTTP errors.
    #[error("failed to fetch prices (signals={signals}): {error}")]
    SendingRequestError {
        #[source]
        error: reqwest::Error,
        signals: String,
    },

    /// Indicates the API returned a non-success HTTP status code.
    #[error("returned HTTP {status} for signals={signals}: {body}")]
    HttpStatusError {
        status: StatusCode,
        body: String,
        signals: String,
    },

    /// Indicates the response body could not be deserialized.
    #[error("failed to parse response for signals={signals}: {source}")]
    ParseResponseError {
        #[source]
        source: reqwest::Error,
        signals: String,
    },

    /// Indicates a failure to parse the API response.
    #[error("invalid price payload for signal {signal}: {source}")]
    ParsePriceError {
        #[source]
        source: ParseError,
        signal: String,
    },
}

/// Errors that can occur while parsing Band API responses.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Indicates that the price field was missing.
    #[error("missing price from signal {0}")]
    MissingPrice(String),
    /// Indicates that the price value is present but not a valid number (NaN/inf).
    #[error("invalid price value {price} from signal {signal}")]
    InvalidPrice { price: f64, signal: String },
    /// Indicates that the timestamp field was missing.
    #[error("missing timestamp from signal {0}")]
    MissingTimestamp(String),
}
