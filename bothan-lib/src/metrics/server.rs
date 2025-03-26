use std::fmt;

use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use tonic::Code;

fn code_to_str(code: Code) -> String {
    match code {
        Code::Ok => "Ok".to_string(),
        Code::Cancelled => "Cancelled".to_string(),
        Code::Unknown => "Unknown".to_string(),
        Code::InvalidArgument => "InvalidArgument".to_string(),
        Code::DeadlineExceeded => "DeadlineExceeded".to_string(),
        Code::NotFound => "NotFound".to_string(),
        Code::AlreadyExists => "AlreadyExists".to_string(),
        Code::PermissionDenied => "PermissionDenied".to_string(),
        Code::ResourceExhausted => "ResourceExhausted".to_string(),
        Code::FailedPrecondition => "FailedPrecondition".to_string(),
        Code::Aborted => "Aborted".to_string(),
        Code::OutOfRange => "OutOfRange".to_string(),
        Code::Unimplemented => "Unimplemented".to_string(),
        Code::Internal => "Internal".to_string(),
        Code::Unavailable => "Unavailable".to_string(),
        Code::DataLoss => "DataLoss".to_string(),
        Code::Unauthenticated => "Unauthenticated".to_string(),
    }
}

pub enum ServiceName {
    GetInfo,
    UpdateRegistry,
    PushMonitoringRecords,
    GetPrices,
}

impl fmt::Display for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ServiceName::GetInfo => "get_info",
            ServiceName::UpdateRegistry => "update_registry",
            ServiceName::PushMonitoringRecords => "push_monitoring_records",
            ServiceName::GetPrices => "get_prices",
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, Debug)]
pub struct ServerMetrics {
    requests_total: Counter<u64>,
    requests_duration: Histogram<u64>,
}

impl ServerMetrics {
    pub fn new() -> Self {
        let meter = global::meter("server");
        Self {
            requests_total: meter
                .u64_counter("server_requests")
                .with_description("total number of requests sent to fetch asset prices")
                .build(),
            requests_duration: meter
                .u64_histogram("server_requests_duration_milliseconds")
                .with_description("time taken to fetch asset prices")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_requests_total(&self, service_name: ServiceName) {
        self.requests_total.add(
            1,
            &[KeyValue::new("service_name", service_name.to_string())],
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
                KeyValue::new("service_name", service_name.to_string()),
                KeyValue::new("status", code_to_str(grpc_code)),
            ],
        );
    }
}
