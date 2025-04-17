// ! Signal definitions for registry entries.
//!
//! This module provides the core structure for defining signals in the registry system.
//! A signal represents a specific asset or data point that can be computed from various
//! sources, processed according to a defined strategy, and optionally transformed through
//! post-processing steps.
//!
//! The module provides:
//!
//! - The [`Signal`] struct which defines the complete specification for a signal
//! - Methods for creating and managing signal definitions
//!
//! # Signal Structure
//!
//! Each signal consists of:
//!
//! - Source queries that specify where to obtain input data
//! - A processor that defines how to combine data from different sources
//! - Optional post-processors that apply transformations to the processed value
//!
//! Signals form the fundamental building blocks of the registry system, enabling
//! the computation of asset information from various data sources in a structured
//! and configurable way.

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::registry::post_processor::PostProcessor;
use crate::registry::processor::Processor;
use crate::registry::source::SourceQuery;

/// A complete definition for computing a signal value.
///
/// The `Signal` struct encapsulates all the information required to compute a signal value:
/// the sources to query for input data, the processing strategy to apply, and any
/// post-processing transformations to perform on the result.
///
/// Signals are the fundamental building blocks of the registry system, allowing for
/// flexible and configurable computation of asset information from various data sources.
///
/// # Components
///
/// * `source_queries` - Definitions of where and how to obtain input data
/// * `processor` - Strategy for combining data from different sources
/// * `post_processors` - Optional transformations to apply to the processed value
///
/// # Examples
///
/// ```
/// use bothan_lib::registry::signal::Signal;
/// use bothan_lib::registry::source::{SourceQuery, Route, RouteOperation};
/// use bothan_lib::registry::processor::{Processor, median::MedianProcessor};
/// use bothan_lib::registry::post_processor::{PostProcessor, tick::TickPostProcessor};
///
/// // Create a BTC-USD signal that uses data from multiple sources
/// let signal = Signal::new(
///     // Source queries
///     vec![
///         SourceQuery::new(
///             "binance".to_string(),
///             "btcusdt".to_string(),
///             vec![
///                 // Apply USDT-USD conversion route
///                 Route::new("USDT-USD".to_string(), RouteOperation::Multiply),
///             ],
///         ),
///         SourceQuery::new(
///             "coinbase".to_string(),
///             "BTC-USD".to_string(),
///             vec![],
///         ),
///     ],
///     // Processor (median with at least 1 source required)
///     Processor::Median(MedianProcessor { min_source_count: 1 }),
///     // Post-processors (convert to tick value)
///     vec![PostProcessor::TickConvertor(TickPostProcessor {})],
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Signal {
    /// Source queries that define where to obtain input data.
    ///
    /// Each query specifies a data source, an identifier within that source,
    /// and optionally a series of routes to apply to the data before processing.
    #[serde(rename = "sources")]
    pub source_queries: Vec<SourceQuery>,
    
    /// The processing strategy to apply to the source data.
    ///
    /// This defines how data from different sources will be combined into a
    /// single output value, such as by taking the median or weighted median.
    pub processor: Processor,
    
    /// Optional post-processing transformations to apply to the processed value.
    ///
    /// These transformations are applied in sequence after the main processing
    /// step, allowing for additional adjustments like tick conversion.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub post_processors: Vec<PostProcessor>,
}

impl Signal {
    /// Creates a new Signal with the specified components.
    ///
    /// This method constructs a complete signal definition from its constituent parts:
    /// the source queries for input data, the processor for combining data, and any
    /// post-processors for transforming the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::registry::signal::Signal;
    /// use bothan_lib::registry::source::SourceQuery;
    /// use bothan_lib::registry::processor::{Processor, median::MedianProcessor};
    /// use bothan_lib::registry::post_processor::PostProcessor;
    ///
    /// // Create a simple signal with a median processor
    /// let signal = Signal::new(
    ///     vec![
    ///         SourceQuery::new("exchange1".to_string(), "btc-usd".to_string(), vec![]),
    ///         SourceQuery::new("exchange2".to_string(), "btc-usd".to_string(), vec![]),
    ///     ],
    ///     Processor::Median(MedianProcessor { min_source_count: 2 }),
    ///     vec![], // No post-processors
    /// );
    /// ```
    pub fn new(
        source_queries: Vec<SourceQuery>,
        processor: Processor,
        post_processors: Vec<PostProcessor>,
    ) -> Self {
        Signal {
            source_queries,
            processor,
            post_processors,
        }
    }
}
