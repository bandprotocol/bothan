use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use tonic::Code;

fn code_to_str(code: Code) -> &'static str {
    match code {
        Code::Ok => "Ok",
        Code::Cancelled => "Cancelled",
        Code::Unknown => "Unknown",
        Code::InvalidArgument => "InvalidArgument",
        Code::DeadlineExceeded => "DeadlineExceeded",
        Code::NotFound => "NotFound",
        Code::AlreadyExists => "AlreadyExists",
        Code::PermissionDenied => "PermissionDenied",
        Code::ResourceExhausted => "ResourceExhausted",
        Code::FailedPrecondition => "FailedPrecondition",
        Code::Aborted => "Aborted",
        Code::OutOfRange => "OutOfRange",
        Code::Unimplemented => "Unimplemented",
        Code::Internal => "Internal",
        Code::Unavailable => "Unavailable",
        Code::DataLoss => "DataLoss",
        Code::Unauthenticated => "Unauthenticated",
    }
}

pub enum ServiceName {
    GetInfo,
    UpdateRegistry,
    PushMonitoringRecords,
    GetPrices,
}

impl ServiceName {
    fn as_str_name(&self) -> &'static str {
        match self {
            ServiceName::GetInfo => "get_info",
            ServiceName::UpdateRegistry => "update_registry",
            ServiceName::PushMonitoringRecords => "push_monitoring_records",
            ServiceName::GetPrices => "get_prices",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerMetrics {
    requests_total: Counter<u64>,
    requests_duration: Histogram<u64>,
}

impl Default for ServerMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerMetrics {
    pub fn new() -> Self {
        let meter = global::meter("server");

        Self {
            requests_total: meter
                .u64_counter("grpc_requests_total")
                .with_description("total number of requests sent to fetch asset prices")
                .build(),
            requests_duration: meter
                .u64_histogram("grpc_requests_duration_milliseconds")
                .with_description("time taken to fetch asset prices")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_requests_total(&self, service_name: ServiceName) {
        self.requests_total.add(
            1,
            &[KeyValue::new("service_name", service_name.as_str_name())],
        );
    }

    pub fn record_requests_duration(
        &self,
        elapsed_time: u64,
        service_name: ServiceName,
        grpc_code: Code,
    ) {
        self.requests_duration.record(
            elapsed_time,
            &[
                KeyValue::new("service_name", service_name.as_str_name()),
                KeyValue::new("status", code_to_str(grpc_code)),
            ],
        );
    }
}
