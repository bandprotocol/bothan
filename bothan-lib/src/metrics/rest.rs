use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};

pub enum PollingResult {
    Success,
    Failed,
    Timeout,
}

impl PollingResult {
    fn as_str_name(&self) -> &'static str {
        match self {
            PollingResult::Success => "success",
            PollingResult::Failed => "failed",
            PollingResult::Timeout => "timeout",
        }
    }
}

#[derive(Clone, Debug)]
pub struct RestMetrics {
    polling_total: Counter<u64>,
    polling_duration: Histogram<u64>,
}

impl Default for RestMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RestMetrics {
    pub fn new() -> Self {
        let meter = global::meter("rest_source");
        Self {
            polling_total: meter
                .u64_counter("rest_polling")
                .with_description("total number of polling")
                .build(),
            polling_duration: meter
                .u64_histogram("rest_polling_duration_milliseconds")
                .with_description("time taken to fetch asset info for each worker")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_polling_total(&self, source: &'static str, status: PollingResult) {
        self.polling_total.add(
            1,
            &[
                KeyValue::new("status", status.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }

    pub fn record_polling_duration(
        &self,
        source: &'static str,
        elapsed_time: u64,
        status: PollingResult,
    ) {
        self.polling_duration.record(
            elapsed_time,
            &[
                KeyValue::new("status", status.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }
}
