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
            .with_boundaries(vec![
                1.0, 2.0, 3.0, 5.0, 10.0, 15.0, 20.0, 25.0, 50.0, 75.0, 100.0, 250.0, 500.0, 750.0,
                1000.0, 2500.0, 5000.0, 10000.0,
            ])
            .build();

        Self {
            operations_total,
            operation_duration,
        }
    }

    pub fn update_store_operation<T: TryInto<u64>>(
        &self,
        source: String,
        elapsed_time: T,
        operation: Operation,
        status: OperationStatus,
    ) -> Result<(), T::Error> {
        let labels = &[
            KeyValue::new("source", source),
            KeyValue::new("operation", operation.to_string()),
            KeyValue::new("status", status.to_string()),
        ];

        self.operations_total.add(1, labels);

        self.operation_duration
            .record(elapsed_time.try_into()?, labels);

        Ok(())
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
    NotFound,
    Failed,
}
