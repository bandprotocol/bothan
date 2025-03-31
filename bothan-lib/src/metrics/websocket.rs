use std::fmt;

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};

pub enum MessageType {
    AssetInfo,
    Ping,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            MessageType::AssetInfo => "asset_info",
            MessageType::Ping => "ping",
        };
        write!(f, "{}", str)
    }
}

pub enum ConnectionResult {
    Success,
    Failed,
}

impl fmt::Display for ConnectionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ConnectionResult::Success => "success",
            ConnectionResult::Failed => "failed",
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, Debug)]
pub struct WebSocketMetrics {
    activity_messages_total: Counter<u64>,
    connection_duration: Histogram<u64>,
    connections_total: Counter<u64>,
}

impl WebSocketMetrics {
    pub fn new(source: &'static str) -> Self {
        let meter = global::meter(source);
        Self {
            activity_messages_total: meter
                .u64_counter("websocket_activity_messages")
                .with_description("total number of messages sent by the source to indicate whether the source is active or not")
                .build(),
            connection_duration: meter
                .u64_histogram("websocket_connection_duration_milliseconds")
                .with_description("time taken for worker to establish a websocket connection to the source")
                .with_unit("milliseconds")
                .build(),
            connections_total: meter
                .u64_counter("websocket_connection")
                .with_description("total number of connections established by a worker to the data source")
                .build()
        }
    }

    pub fn increment_activity_messages_total(&self, worker: String, message: MessageType) {
        self.activity_messages_total.add(
            1,
            &[
                KeyValue::new("worker", worker),
                KeyValue::new("message_type", message.to_string()),
            ],
        );
    }

    pub fn record_connection_duration(
        &self,
        elapsed_time: u64,
        worker: String,
        status: ConnectionResult,
    ) {
        self.connection_duration.record(
            elapsed_time,
            &[
                KeyValue::new("worker", worker),
                KeyValue::new("status", status.to_string()),
            ],
        );
    }

    pub fn increment_connections_total(&self, worker: String, status: ConnectionResult) {
        self.connections_total.add(
            1,
            &[
                KeyValue::new("worker", worker),
                KeyValue::new("status", status.to_string()),
            ],
        );
    }
}
