use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;

#[derive(Clone, Debug)]
pub struct Metrics {
    operations_total: Counter<u64>,
    operation_duration: Histogram<u64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("store");

        let operations_total = meter
            .u64_counter("store_operations")
            .with_description("Total number of operations executed by the store")
            .build();
        let operation_duration = meter
            .u64_histogram("store_operation_duration_microseconds")
            .with_description("Time taken to execute each store operation")
            .with_unit("microseconds")
            .build();

        Self {
            operations_total,
            operation_duration,
        }
    }

    pub fn update_store_operation(
        &self,
        source: String,
        elapsed_time: u128,
        operation: Operation,
        status: OperationStatus,
    ) {
        let labels = &[
            KeyValue::new("source", source),
            KeyValue::new("operation", operation.to_string()),
            KeyValue::new("status", status.to_string()),
        ];
        self.operations_total.add(1, labels);
        // `elapsed_time` is u128, but it will never exceed u64::MAX in practice
        self.operation_duration.record(elapsed_time as u64, labels);
    }
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum Operation {
    GetAssetInfo,
    InsertBatchAssetInfo,
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum OperationStatus {
    Success,
    Stale,
    NotFound,
    Failed,
}
