use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;
use tonic::Code;

use crate::metrics::utils::code_to_str;

#[derive(Clone, Debug)]
pub struct Metrics {
    requests_total: Counter<u64>,
    request_duration: Histogram<u64>,
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
            .with_description("total number of requests sent to fetch asset prices")
            .build();
        let request_duration = meter
            .u64_histogram("server_request_duration_milliseconds")
            .with_description("time taken to fetch asset prices")
            .with_unit("milliseconds")
            .build();

        Self {
            requests_total,
            request_duration,
        }
    }

    pub fn increment_requests_total(&self, service_name: ServiceName) {
        self.requests_total.add(
            1,
            &[KeyValue::new("service_name", service_name.to_string())],
        );
    }

    pub fn record_requests_duration<T: TryInto<u64>>(
        &self,
        elapsed_time: T,
        service_name: ServiceName,
        grpc_code: Code,
    ) -> Result<(), T::Error> {
        self.request_duration.record(
            elapsed_time.try_into()?,
            &[
                KeyValue::new("service_name", service_name.to_string()),
                KeyValue::new("status", code_to_str(grpc_code)),
            ],
        );
        Ok(())
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
