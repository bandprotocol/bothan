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

pub enum ConnectionStatus {
    Success,
    Failed,
}

impl ConnectionStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConnectionStatus::Success => "success",
            ConnectionStatus::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebSocketMetrics {
    source_activity_count: Counter<u64>,
    source_connection_time: Histogram<u64>,
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
            source_activity_count: meter
                .u64_counter("source_activity_message_count")
                .with_description("total number of messages sent by the source to indicate whether the source is active or not")
                .build(),
            source_connection_time: meter
                .u64_histogram("source_connection_time")
                .with_description("time taken for worker to establish a websocket connection to the source.")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_source_activity_message_count(
        &self,
        source: &'static str,
        message: MessageType,
    ) {
        self.source_activity_count.add(
            1,
            &[
                KeyValue::new("messsage_type", message.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }

    pub fn record_source_connection_time(
        &self,
        source: &'static str,
        elapsed_time: u64,
        status: ConnectionStatus,
    ) {
        self.source_connection_time.record(
            elapsed_time,
            &[
                KeyValue::new("status", status.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }
}
