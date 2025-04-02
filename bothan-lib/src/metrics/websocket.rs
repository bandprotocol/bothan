use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;
use tracing::warn;

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum MessageType {
    AssetInfo,
    Ping,
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum ConnectionResult {
    Success,
    Failed,
}

#[derive(Clone, Debug)]
pub struct WebSocketMetrics {
    worker: String,
    activity_messages_total: Counter<u64>,
    connection_duration: Histogram<u64>,
    connections_total: Counter<u64>,
}

impl WebSocketMetrics {
    pub fn new(source: &'static str) -> Self {
        Self::new_with_worker(source, source.to_string())
    }

    pub fn new_with_worker(source: &'static str, worker: String) -> Self {
        let meter = global::meter(source);

        Self {
            worker,
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

    pub fn increment_activity_messages_total(&self, message: MessageType) {
        self.activity_messages_total.add(
            1,
            &[
                KeyValue::new("worker", self.worker.clone()),
                KeyValue::new("message_type", message.to_string()),
            ],
        );
    }

    pub fn record_connection_duration<T>(&self, elapsed_time: T, status: ConnectionResult)
    where
        T: TryInto<u64>,
        T::Error: std::fmt::Display,
    {
        match elapsed_time.try_into() {
            Ok(elapsed_time) => self.connection_duration.record(
                elapsed_time,
                &[
                    KeyValue::new("worker", self.worker.clone()),
                    KeyValue::new("status", status.to_string()),
                ],
            ),
            Err(e) => warn!("failed to record connection duration: {}", e),
        }
    }

    pub fn increment_connections_total(&self, status: ConnectionResult) {
        self.connections_total.add(
            1,
            &[
                KeyValue::new("worker", self.worker.clone()),
                KeyValue::new("status", status.to_string()),
            ],
        );
    }
}
