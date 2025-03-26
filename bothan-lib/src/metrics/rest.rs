use std::fmt;

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};

pub enum PollingResult {
    Success,
    Failed,
    Timeout,
}

impl fmt::Display for PollingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            PollingResult::Success => "success",
            PollingResult::Failed => "failed",
            PollingResult::Timeout => "timeout",
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, Debug)]
pub struct RestMetrics {
    polling_total: Counter<u64>,
    polling_duration: Histogram<u64>,
}

impl RestMetrics {
    pub fn new(source: &'static str) -> Self {
        let meter = global::meter(source);
        Self {
            polling_total: meter
                .u64_counter("rest_polling")
                .with_description("total number of polling requests sent by the worker to the source")
                .build(),
            polling_duration: meter
                .u64_histogram("rest_polling_duration_milliseconds")
                .with_description("duration of each polling request made by the worker to fetch asset info")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_polling_total(&self, status: PollingResult) {
        self.polling_total
            .add(1, &[KeyValue::new("status", status.to_string())]);
    }

    pub fn record_polling_duration(&self, elapsed_time: u64, status: PollingResult) {
        self.polling_duration
            .record(elapsed_time, &[KeyValue::new("status", status.to_string())]);
    }
}
