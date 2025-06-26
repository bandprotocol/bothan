//! Metrics collection for gRPC server operations.
//!
//! This module provides the [`Metrics`] struct and related types for tracking gRPC server statistics
//! such as the number of requests and their processing durations. It leverages OpenTelemetry for metrics
//! instrumentation, supporting monitoring and observability.

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;
use tonic::Code;

use crate::metrics::utils::code_to_str;

/// Holds counters and histograms for gRPC server metrics.
#[derive(Clone, Debug)]
pub struct Metrics {
    /// Counter tracking total server requests.
    requests_total: Counter<u64>,

    /// Histogram recording request durations in milliseconds.
    request_duration: Histogram<f64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    /// Creates a new [`Metrics`] instance configured for the gRPC server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_lib::metrics::server::Metrics;
    /// use bothan_lib::metrics::server::ServiceName;
    /// use tonic::Code;
    ///
    /// let metrics = Metrics::new();
    /// metrics.update_server_request(42.0, ServiceName::GetInfo, Code::Ok);
    /// ```
    pub fn new() -> Self {
        let meter = global::meter("server");

        let requests_total = meter
            .u64_counter("server_requests")
            .with_description("Total number of requests sent to the server")
            .build();
        let request_duration = meter
            .f64_histogram("server_request_duration_milliseconds")
            .with_description("Time taken to process each request sent to the server")
            .with_unit("milliseconds")
            .build();

        Self {
            requests_total,
            request_duration,
        }
    }

    /// Records a server request result and duration.
    ///
    /// # Arguments
    ///
    /// * `elapsed_time` - Duration of the request in milliseconds.
    /// * `service_name` - The gRPC service being called.
    /// * `grpc_code` - The gRPC status code returned.
    pub fn update_server_request(
        &self,
        elapsed_time: f64,
        service_name: ServiceName,
        grpc_code: Code,
    ) {
        let labels = &[
            KeyValue::new("service_name", service_name.to_string()),
            KeyValue::new("status", code_to_str(grpc_code)),
        ];
        self.requests_total.add(1, labels);
        self.request_duration.record(elapsed_time, labels);
    }
}

/// Possible gRPC service endpoints for server requests.
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum ServiceName {
    /// Get information from the server.
    GetInfo,
    /// Update the registry on the server.
    UpdateRegistry,
    /// Push monitoring records to the server.
    PushMonitoringRecords,
    /// Get price data from the server.
    GetPrices,
}
