use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;

#[derive(Clone, Debug)]
pub struct Metrics {
    polling_total: Counter<u64>,
    polling_duration: Histogram<u64>,
}

impl Metrics {
    pub fn new(source: &'static str) -> Self {
        let meter = global::meter(source);

        let polling_total = meter
            .u64_counter("rest_polling")
            .with_description("Total number of polling requests sent by the worker to the source")
            .build();
        let polling_duration = meter
            .u64_histogram("rest_polling_duration_milliseconds")
            .with_description(
                "Time taken by the worker to complete each polling request to fetch asset info",
            )
            .with_unit("milliseconds")
            .build();

        Self {
            polling_total,
            polling_duration,
        }
    }

    pub fn update_rest_polling(
        &self,
        elapsed_time: u128,
        status: PollingResult,
    ){
        let labels = &[KeyValue::new("status", status.to_string())];
        self.polling_total.add(1, labels);
        // `elapsed_time` is u128, but it will never exceed u64::MAX in practice
        self.polling_duration
            .record(elapsed_time as u64, labels);
    }
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum PollingResult {
    Success,
    Failed,
    Timeout,
}
