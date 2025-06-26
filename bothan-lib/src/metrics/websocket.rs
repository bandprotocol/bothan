//! Metrics collection for websocket operations.
//!
//! This module provides the [`Metrics`] struct and related types for tracking websocket connection and message statistics
//! such as the number of activity messages, connection attempts, and their durations. It leverages OpenTelemetry for metrics
//! instrumentation, supporting monitoring and observability.

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;

/// Holds counters and histograms for websocket metrics.
#[derive(Clone, Debug)]
pub struct Metrics {
    /// The worker identifier for labeling metrics.
    worker: String,
    /// Counter tracking total activity messages sent.
    activity_messages_total: Counter<u64>,
    /// Histogram recording connection durations in milliseconds.
    connection_duration: Histogram<u64>,
    /// Counter tracking total websocket connections established.
    connections_total: Counter<u64>,
}

impl Metrics {
    /// Creates a new [`Metrics`] instance configured for a specified source and worker.
    ///
    /// # Arguments
    ///
    /// * `source` - A static string identifying the source whose metrics are being recorded.
    /// * `worker` - The worker identifier for labeling metrics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bothan_lib::metrics::websocket::{Metrics, MessageType, ConnectionResult};
    ///
    /// let metrics = Metrics::new("example_source", "worker-1".to_string());
    /// metrics.increment_activity_messages_total(MessageType::Ping);
    /// metrics.update_websocket_connection(200, ConnectionResult::Success);
    /// ```
    pub fn new(source: &'static str, worker: String) -> Self {
        let meter = global::meter(source);

        let activity_messages_total = meter
            .u64_counter("websocket_activity_messages")
            .with_description("Total number of messages sent by the source to indicate whether the source is active or not")
            .build();
        let connection_duration = meter
            .u64_histogram("websocket_connection_duration_milliseconds")
            .with_description(
                "Time taken for worker to establish a websocket connection to the source",
            )
            .with_unit("milliseconds")
            .build();
        let connections_total = meter
            .u64_counter("websocket_connection")
            .with_description("Total number of connections established by a worker to the source")
            .build();

        Self {
            worker,
            activity_messages_total,
            connection_duration,
            connections_total,
        }
    }

    /// Increments the activity message counter for a given message type.
    ///
    /// # Arguments
    ///
    /// * `message` - The type of activity message sent.
    pub fn increment_activity_messages_total(&self, message: MessageType) {
        self.activity_messages_total.add(
            1,
            &[
                KeyValue::new("worker", self.worker.clone()),
                KeyValue::new("message_type", message.to_string()),
            ],
        );
    }

    /// Records a websocket connection attempt and its duration.
    ///
    /// # Arguments
    ///
    /// * `elapsed_time` - Duration of the connection attempt in milliseconds.
    /// * `status` - The result of the connection attempt.
    pub fn update_websocket_connection(&self, elapsed_time: u128, status: ConnectionResult) {
        let labels = &[
            KeyValue::new("worker", self.worker.clone()),
            KeyValue::new("status", status.to_string()),
        ];
        self.connections_total.add(1, labels);
        // `elapsed_time` is u128, but it will never exceed u64::MAX in practice
        self.connection_duration.record(elapsed_time as u64, labels);
    }
}

/// Possible types of websocket activity messages.
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum MessageType {
    /// Asset information message.
    AssetInfo,
    /// Ping message.
    Ping,
    /// Unused message type.
    Unused,
    /// Error message.
    Error,
}

/// Possible results for a websocket connection attempt.
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum ConnectionResult {
    /// The connection was successful.
    Success,
    /// The connection failed.
    Failed,
}
