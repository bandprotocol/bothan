use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};

pub enum MessageType {
    AssetInfo,
    Ping,
}

impl MessageType {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageType::AssetInfo => "asset_info",
            MessageType::Ping => "ping",
        }
    }
}

pub enum ConnectionResult {
    Success,
    Failed,
}

impl ConnectionResult {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionResult::Success => "success",
            ConnectionResult::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebSocketMetrics {
    activity_messages_total: Counter<u64>,
    connection_duration: Histogram<u64>,
}

impl Default for WebSocketMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketMetrics {
    pub fn new() -> Self {
        let meter = global::meter("websocket_source");
        Self {
            activity_messages_total: meter
                .u64_counter("websocket_activity_messages")
                .with_description("total number of messages sent by the source to indicate whether the source is active or not")
                .build(),
            connection_duration: meter
                .u64_histogram("websocket_connection_duration_milliseconds")
                .with_description("time taken for worker to establish a websocket connection to the source.")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_activity_messages_total(
        &self,
        source: &'static str,
        message: MessageType,
    ) {
        self.activity_messages_total.add(
            1,
            &[
                KeyValue::new("messsage_type", message.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }

    pub fn record_connection_duration(
        &self,
        source: &'static str,
        elapsed_time: u64,
        status: ConnectionResult,
    ) {
        self.connection_duration.record(
            elapsed_time,
            &[
                KeyValue::new("status", status.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }
}
