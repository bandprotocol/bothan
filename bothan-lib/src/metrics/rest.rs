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
            .with_description("total number of polling requests sent by the worker to the source")
            .build();
        let polling_duration = meter
            .u64_histogram("rest_polling_duration_milliseconds")
            .with_description(
                "duration of each polling request made by the worker to fetch asset info",
            )
            .with_unit("milliseconds")
            .build();

        Self {
            polling_total,
            polling_duration,
        }
    }

    pub fn increment_polling_total(&self, status: PollingResult) {
        self.polling_total
            .add(1, &[KeyValue::new("status", status.to_string())]);
    }

    pub fn record_polling_duration<T: TryInto<u64>>(
        &self,
        elapsed_time: T,
        status: PollingResult,
    ) -> Result<(), T::Error> {
        self.polling_duration.record(
            elapsed_time.try_into()?,
            &[KeyValue::new("status", status.to_string())],
        );
        Ok(())
    }
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum PollingResult {
    Success,
    Failed,
    Timeout,
}
