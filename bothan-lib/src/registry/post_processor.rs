//! Post-processing transformations for signal values.
//!
//! This module provides post-processors that apply transformations to signal values
//! after they have been processed from source data. Post-processors can be used to
//! adjust, normalize, or otherwise transform the values produced by the main processor.
//!
//! The module provides:
//!
//! - The [`PostProcessor`] enum which represents different post-processing strategies
//! - Specialized post-processor implementations in submodules
//! - Error handling for the post-processing operations
//!
//! # Available Post-Processors
//!
//! The following post-processing strategies are available:
//!
//! - [`TickConvertor`](tick::TickPostProcessor) - Converts values to specific tick sizes
//!
//! # Extensibility
//!
//! This module is designed to be extensible. New post-processing strategies can be added by:
//!
//! 1. Creating a new post-processor implementation in a submodule
//! 2. Adding a new variant to the [`PostProcessor`] enum
//! 3. Implementing the necessary logic in the `name` and `post_process` methods
//!
//! # Usage in Registry
//!
//! Post-processors are used within the registry system to define how signal values
//! should be transformed after the main processing step. Each signal can specify
//! multiple post-processors that are applied in sequence.

use bincode::{Decode, Encode};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod tick;

/// Error type for post-processor operations.
///
/// This error is returned when a post-processor encounters an issue while
/// transforming a value, such as invalid input or mathematical errors.
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::post_processor::PostProcessError;
///
/// let error = PostProcessError::new("Invalid tick size");
/// assert_eq!(error.to_string(), "Invalid tick size");
/// ```
#[derive(Clone, Debug, Error, PartialEq, Serialize, Deserialize)]
#[error("{msg}")]
pub struct PostProcessError {
    msg: String,
}

impl PostProcessError {
    /// Creates a new PostProcessError with the specified message.
    pub fn new<T: Into<String>>(msg: T) -> Self {
        PostProcessError { msg: msg.into() }
    }
}

/// Represents different strategies for post-processing signal values.
///
/// The `PostProcessor` enum encapsulates different algorithms for transforming
/// signal values after they have been processed from source data. Each variant
/// contains its own configuration parameters.
///
/// # Variants
///
/// * `TickConvertor` - Converts values to specific tick sizes
///
/// # Examples
///
/// Creating a tick convertor post-processor:
///
/// ```
/// use bothan_lib::registry::post_processor::{PostProcessor, tick::TickPostProcessor};
/// use serde_json::json;
///
/// // Create a post-processor from JSON
/// let json_data = json!({
///     "function": "tick_convertor",
///     "params": {}
/// });
///
/// let post_processor: PostProcessor = serde_json::from_value(json_data).unwrap();
/// assert_eq!(post_processor.name(), "tick_convertor");
/// ```
///
/// Using a post-processor:
///
/// ```
/// use bothan_lib::registry::post_processor::{PostProcessor, tick::TickPostProcessor};
/// use rust_decimal::Decimal;
///
/// // Create a tick convertor post-processor
/// let post_processor = PostProcessor::TickConvertor(TickPostProcessor {});
///
/// // Apply the post-processor to a value
/// let value = Decimal::new(1234, 0);
/// let result = post_processor.post_process(value).unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
pub enum PostProcessor {
    /// Tick convertor that adjusts values to specific tick sizes.
    TickConvertor(tick::TickPostProcessor),
}

impl PostProcessor {
    /// Returns the name of the post-processor.
    ///
    /// This method returns a string identifier for the post-processor type,
    /// which can be used for logging, debugging, or serialization purposes.
    pub fn name(&self) -> &str {
        match self {
            PostProcessor::TickConvertor(_) => "tick_convertor",
        }
    }

    /// Applies the post-processor to a value.
    ///
    /// This method applies the post-processor's transformation algorithm
    /// to the input value to produce a transformed output value.
    ///
    /// # Errors
    ///
    /// Returns a `PostProcessError` if the transformation operation fails, such as when:
    /// - The input value is invalid for the transformation
    /// - A mathematical error occurs during transformation
    pub fn post_process(&self, data: Decimal) -> Result<Decimal, PostProcessError> {
        match self {
            PostProcessor::TickConvertor(tick) => tick.process(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use tick::TickPostProcessor;

    use super::*;

    #[test]
    fn test_deserialize() {
        let json_str = r#"{ "function": "tick_convertor", "params": { } }"#;
        let expected_function = PostProcessor::TickConvertor(TickPostProcessor {});

        let deserialized = serde_json::from_str::<PostProcessor>(json_str);
        assert_eq!(deserialized.unwrap(), expected_function);
    }

    #[test]
    fn test_deserialize_with_invalid_parameter() {
        let json_str = r#"{ "function": "median", "params": { "test": "Jesus" } }"#;

        let deserialized = serde_json::from_str::<PostProcessor>(json_str);
        assert!(deserialized.is_err());
    }
}
