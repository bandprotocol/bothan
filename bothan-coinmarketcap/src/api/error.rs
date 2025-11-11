//! Error types for CoinMarketCap REST API client operations.
//!
//! This module provides custom error types used throughout the CoinMarketCap REST API integration,
//! particularly for REST API client configuration and concurrent background data fetching.

use reqwest::StatusCode;
use thiserror::Error;

/// Errors from initializing the CoinMarketCap REST API builder.
///
/// These errors can occur during the initialization and configuration of the HTTP client
/// or while constructing request parameters.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Indicates the API key is missing.
    #[error("missing api key")]
    MissingAPIKey,

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

/// General errors from CoinMarketCap API operations.
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

/// Errors from fetching and handling data from the CoinMarketCap REST API.
///
/// These errors typically occur during API calls, response parsing, or data validation.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Indicates that an ID in the request is not a valid integer.
    #[error("ids contains non integer value: {0}")]
    InvalidId(String),

    /// Indicates HTTP request failure due to network issues or HTTP errors.
    #[error("failed to fetch quotes (ids={ids}): {error}")]
    SendingRequestError {
        #[source]
        error: reqwest::Error,
        ids: String,
    },

    /// Indicates a non-success HTTP status code.
    #[error("returned HTTP {status} for ids={ids}: {body}")]
    HttpStatusError {
        status: StatusCode,
        body: String,
        ids: String,
    },

    /// Indicates the response body could not be parsed into the expected shape.
    #[error("failed to parse response for ids={ids}: {source}")]
    ParseResponseError {
        #[source]
        source: reqwest::Error,
        ids: String,
    },

    /// Indicates the ticker data contains invalid values for a specific asset.
    #[error("invalid quote for id {id}: {source}")]
    InvalidQuote {
        #[source]
        source: ParseError,
        id: String,
    },
}

/// Errors that can occur while parsing CoinMarketCap API responses.
#[derive(Debug, Error)]
pub enum ParseError {
    /// Indicates that no price value was found in the response.
    #[error("no price value was found")]
    MissingPrice,
    /// Indicates that the price value is not a valid number (NaN).
    #[error("price is NaN")]
    InvalidPrice,
    /// Indicates that a datetime string could not be parsed.
    #[error("not a valid datetime: {0}")]
    InvalidDatetime(#[from] chrono::ParseError),
}
