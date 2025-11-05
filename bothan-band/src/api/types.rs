//! Data types for interacting with the Band REST API.
//!
//! This module provides types for deserializing responses from the Band REST API.
//!
use serde::{Deserialize, Serialize};

/// Represents price and market data for a single asset in USD.
///
/// `Price` contains fields matching those returned by the [Band api endpoint].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub signal: String,
    pub price: Option<f64>,
    pub timestamp: Option<i64>,
}
