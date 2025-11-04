//! Data types for interacting with the Band REST API.
//!
//! This module provides types for deserializing responses from the Band REST API.
//!
use serde::{Deserialize, Serialize};

/// The base URL for the Band API.
/// Needs to change to the actual Band source API URL.
pub(crate) const DEFAULT_URL: &str = "https://bandsource-url.com";

/// Represents price and market data for a single asset in USD.
///
/// `Price` contains fields matching those returned by the [Band api endpoint].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub signal: String,
    pub price: f64,
    pub timestamp: i64,
}