use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;
use tracing::warn;

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum PollingResult {
    Success,
    Failed,
    Timeout,
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
                .with_description(
                    "total number of polling requests sent by the worker to the source",
                )
                .build(),
            polling_duration: meter
                .u64_histogram("rest_polling_duration_milliseconds")
                .with_description(
                    "duration of each polling request made by the worker to fetch asset info",
                )
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_polling_total(&self, status: PollingResult) {
        self.polling_total
            .add(1, &[KeyValue::new("status", status.to_string())]);
    }

    pub fn record_polling_duration<T>(&self, elapsed_time: T, status: PollingResult)
    where
        T: TryInto<u64>,
        T::Error: std::fmt::Display,
    {
        match elapsed_time.try_into() {
            Ok(elapsed_time) => self
                .polling_duration
                .record(elapsed_time, &[KeyValue::new("status", status.to_string())]),
            Err(e) => warn!("failed to record polling duration: {}", e),
        }
    }
}
