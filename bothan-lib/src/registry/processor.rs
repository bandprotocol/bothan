//! Processing strategies for combining source data into signal values.
//!
//! This module provides processors that transform multiple input values from
//! various sources into a single output value. Each processor implements a different
//! strategy for combining these values, such as taking the median or weighted median.
//!
//! The module provides:
//!
//! - The [`Processor`] enum which represents different processing strategies
//! - Specialized processor implementations in submodules
//! - Error handling for the processing operations
//!
//! # Available Processors
//!
//! The following processing strategies are available:
//!
//! - [`Median`](median::MedianProcessor) - Computes the median of the input values
//! - [`WeightedMedian`](weighted_median::WeightedMedianProcessor) - Computes a weighted median based on configurable weights
//!
//! # Extensibility
//!
//! This module is designed to be extensible. New processing strategies can be added by:
//!
//! 1. Creating a new processor implementation in a submodule
//! 2. Adding a new variant to the [`Processor`] enum
//! 3. Implementing the necessary logic in the `name` and `process` methods
//!
//! # Usage in Registry
//!
//! Processors are used within the registry system to define how source data should be
//! combined into a single value for each signal. Each signal specifies a processor
//! and its configuration parameters.

use bincode::{Decode, Encode};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod median;
pub mod weighted_median;

/// Error type for processor operations.
///
/// This error is returned when a processor encounters an issue while
/// processing data, such as insufficient data points or mathematical errors.
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::processor::ProcessError;
///
/// let error = ProcessError::new("Insufficient data points");
/// assert_eq!(error.to_string(), "Insufficient data points");
/// ```
#[derive(Clone, Debug, Error, PartialEq, Serialize, Deserialize)]
#[error("{msg}")]
pub struct ProcessError {
    msg: String,
}

impl ProcessError {
    /// Creates a new ProcessError with the specified message.
    pub fn new<T: Into<String>>(msg: T) -> Self {
        ProcessError { msg: msg.into() }
    }
}

/// Represents different strategies for processing source data into a signal value.
///
/// The `Processor` enum encapsulates different algorithms for combining multiple
/// input values from various sources into a single output value. Each variant
/// contains its own configuration parameters.
///
/// # Variants
///
/// * `Median` - Computes the median of the input values
/// * `WeightedMedian` - Computes a weighted median based on configured weights
///
/// # Examples
///
/// Creating a median processor:
///
/// ```
/// use bothan_lib::registry::processor::{Processor, median::MedianProcessor};
/// use serde_json::json;
///
/// // Create a processor from JSON
/// let json_data = json!({
///     "function": "median",
///     "params": {
///         "min_source_count": 3
///     }
/// });
///
/// let processor: Processor = serde_json::from_value(json_data).unwrap();
/// assert_eq!(processor.name(), "median");
/// ```
///
/// Using a processor:
///
/// ```
/// use bothan_lib::registry::processor::{Processor, median::MedianProcessor};
/// use rust_decimal::Decimal;
///
/// // Create a median processor that requires at least 3 sources
/// let processor = Processor::Median(MedianProcessor { min_source_count: 3 });
///
/// // Process some data
/// let data = vec![
///     ("source1".to_string(), Decimal::new(100, 0)),
///     ("source2".to_string(), Decimal::new(200, 0)),
///     ("source3".to_string(), Decimal::new(300, 0)),
/// ];
///
/// let result = processor.process(data).unwrap();
/// assert_eq!(result, Decimal::new(200, 0));
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
pub enum Processor {
    /// Median processor that computes the median of input values.
    Median(median::MedianProcessor),
    
    /// Weighted median processor that applies weights to input values.
    WeightedMedian(weighted_median::WeightedMedianProcessor),
}

impl Processor {
    /// Returns the name of the processor.
    ///
    /// This method returns a string identifier for the processor type,
    /// which can be used for logging, debugging, or serialization purposes.
    pub fn name(&self) -> &str {
        match self {
            Processor::Median(_) => "median",
            Processor::WeightedMedian(_) => "weighted_median",
        }
    }

    /// Processes input data into a single output value.
    ///
    /// This method applies the processor's algorithm to the input data
    /// to produce a single output value. The input data consists of pairs
    /// of source identifiers and their corresponding values.
    ///
    /// # Errors
    ///
    /// Returns a `ProcessError` if the processing operation fails, such as when:
    /// - There are insufficient data points
    /// - The data does not meet the processor's requirements
    /// - A mathematical error occurs during processing
    pub fn process(&self, data: Vec<(String, Decimal)>) -> Result<Decimal, ProcessError> {
        match self {
            Processor::Median(median) => {
                let data = data.into_iter().map(|(_, value)| value).collect();
                median.process(data)
            }
            Processor::WeightedMedian(weighted_median) => weighted_median.process(data),
        }
    }
}
