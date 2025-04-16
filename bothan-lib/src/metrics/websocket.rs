use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;

#[derive(Clone, Debug)]
pub struct Metrics {
    worker: String,
    activity_messages_total: Counter<u64>,
    connection_duration: Histogram<u64>,
    connections_total: Counter<u64>,
}

impl Metrics {
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

    pub fn increment_activity_messages_total(&self, message: MessageType) {
        self.activity_messages_total.add(
            1,
            &[
                KeyValue::new("worker", self.worker.clone()),
                KeyValue::new("message_type", message.to_string()),
            ],
        );
    }

    pub fn update_websocket_connection<T: TryInto<u64>>(
        &self,
        elapsed_time: T,
        status: ConnectionResult,
    ) -> Result<(), T::Error> {
        let labels = &[
            KeyValue::new("worker", self.worker.clone()),
            KeyValue::new("status", status.to_string()),
        ];
        self.connections_total.add(1, labels);
        self.connection_duration
            .record(elapsed_time.try_into()?, labels);
        Ok(())
    }
}

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
