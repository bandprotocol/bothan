use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;
use tonic::Code;

use crate::metrics::utils::code_to_str;

#[derive(Clone, Debug)]
pub struct Metrics {
    requests_total: Counter<u64>,
    request_duration: Histogram<f64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let meter = global::meter("server");

        let requests_total = meter
            .u64_counter("server_requests")
            .with_description("Total number of requests sent to the server")
            .build();
        let request_duration = meter
            .f64_histogram("server_request_duration_milliseconds")
            .with_description("Time taken to process each request sent to the server")
            .with_unit("milliseconds")
            .with_boundaries(vec![
                0.1, 1.0, 2.0, 3.0, 5.0, 10.0, 15.0, 20.0, 25.0, 50.0, 75.0, 100.0, 250.0, 500.0,
                750.0, 1000.0, 2500.0, 5000.0, 10000.0,
            ])
            .build();

        Self {
            requests_total,
            request_duration,
        }
    }

    pub fn update_server_request(
        &self,
        elapsed_time: f64,
        service_name: ServiceName,
        grpc_code: Code,
    ) {
        let labels = &[
            KeyValue::new("service_name", service_name.to_string()),
            KeyValue::new("status", code_to_str(grpc_code)),
        ];
        self.requests_total.add(1, labels);
        self.request_duration.record(elapsed_time, labels);
    }
}

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum ServiceName {
    GetInfo,
    UpdateRegistry,
    PushMonitoringRecords,
    GetPrices,
}
