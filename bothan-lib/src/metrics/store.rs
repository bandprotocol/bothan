//! Metrics collection for store operations.
//!
//! This module provides the [`Metrics`] struct and related types for tracking store operation statistics
//! such as the number of operations and their execution durations. It leverages OpenTelemetry for metrics
//! instrumentation, supporting monitoring and observability.

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;

/// Holds counters and histograms for store operation metrics.
#[derive(Clone, Debug)]
pub struct Metrics {
    /// Counter tracking total store operations.
    operations_total: Counter<u64>,

    /// Histogram recording operation durations in microseconds.
    operation_duration: Histogram<u64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    /// Creates a new [`Metrics`] instance configured for the store.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_lib::metrics::store::{Metrics, Operation, OperationStatus};
    ///
    /// let metrics = Metrics::new();
    /// metrics.update_store_operation("source".to_string(), 100, Operation::GetAssetInfo, OperationStatus::Success);
    /// ```
    pub fn new() -> Self {
        let meter = global::meter("store");

        let operations_total = meter
            .u64_counter("store_operations")
            .with_description("Total number of operations executed by the store")
            .build();
        let operation_duration = meter
            .u64_histogram("store_operation_duration_microseconds")
            .with_description("Time taken to execute each store operation")
            .with_unit("microseconds")
            .build();

        Self {
            operations_total,
            operation_duration,
        }
    }

    /// Records a store operation result and duration.
    ///
    /// # Arguments
    ///
    /// * `source` - The source of the operation.
    /// * `elapsed_time` - Duration of the operation in microseconds.
    /// * `operation` - The type of operation performed.
    /// * `status` - The result of the operation.
    pub fn update_store_operation(
        &self,
        source: String,
        elapsed_time: u128,
        operation: Operation,
        status: OperationStatus,
    ) {
        let labels = &[
            KeyValue::new("source", source),
            KeyValue::new("operation", operation.to_string()),
            KeyValue::new("status", status.to_string()),
        ];
        self.operations_total.add(1, labels);
        // `elapsed_time` is u128, but it will never exceed u64::MAX in practice
        self.operation_duration.record(elapsed_time as u64, labels);
    }
}

/// Possible store operations.
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum Operation {
    /// Retrieve asset information from the store.
    GetAssetInfo,
    /// Insert a batch of asset information into the store.
    InsertBatchAssetInfo,
}

/// Possible results for a store operation.
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum OperationStatus {
    /// The operation completed successfully.
    Success,
    /// The operation completed but the data was stale.
    Stale,
    /// The requested data was not found.
    NotFound,
    /// The operation failed.
    Failed,
}
