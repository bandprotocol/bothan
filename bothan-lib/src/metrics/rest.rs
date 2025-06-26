//! Metrics collection for REST polling operations.
//!
//! This module provides the [`Metrics`] struct and related types for tracking REST polling statistics
//! such as the number of polling requests and their durations. It leverages OpenTelemetry for metrics
//! instrumentation, supporting monitoring and observability.

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;

/// Holds counters and histograms for REST polling metrics.
#[derive(Clone, Debug)]
pub struct Metrics {
    /// Counter tracking total polling requests.
    polling_total: Counter<u64>,

    /// Histogram recording polling durations in milliseconds.
    polling_duration: Histogram<u64>,
}

impl Metrics {
    /// Creates a new [`Metrics`] instance configured for a specified source.
    ///
    /// # Arguments
    ///
    /// * `source` - A string identifying the source whose metrics are being recorded.
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_lib::metrics::rest::Metrics;
    /// use bothan_lib::metrics::rest::PollingResult;
    ///
    /// let metrics = Metrics::new("example_source");
    /// metrics.update_rest_polling(123, PollingResult::Success);
    /// ```
    pub fn new(source: &'static str) -> Self {
        let meter = global::meter(source);

        let polling_total = meter
            .u64_counter("rest_polling")
            .with_description("Total number of polling requests sent by the worker to the source")
            .build();

        let polling_duration = meter
            .u64_histogram("rest_polling_duration_milliseconds")
            .with_description(
                "Time taken by the worker to complete each polling request to fetch asset info",
            )
            .with_unit("milliseconds")
            .build();

        Self {
            polling_total,
            polling_duration,
        }
    }

    /// Records a polling request result and duration.
    ///
    /// # Arguments
    ///
    /// * `elapsed_time` - Duration of the polling request in milliseconds.
    /// * `status` - The result of the polling request (success, failure, or timeout).
    pub fn update_rest_polling(&self, elapsed_time: u128, status: PollingResult) {
        let labels = &[KeyValue::new("status", status.to_string())];
        self.polling_total.add(1, labels);
        self.polling_duration.record(elapsed_time as u64, labels);
    }
}

/// Possible results for a REST polling operation.
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum PollingResult {
    /// Indicates the polling request completed successfully.
    Success,

    /// Indicates the polling request encountered an error.
    Failed,

    /// Indicates the polling request did not complete within the expected polling duration.
    Timeout,
}
