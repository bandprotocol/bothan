use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};
use strum_macros::Display;
use tonic::Code;
use tracing::warn;

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

#[derive(Display)]
#[strum(serialize_all = "snake_case")]
pub enum ServiceName {
    GetInfo,
    UpdateRegistry,
    PushMonitoringRecords,
    GetPrices,
}

#[derive(Clone, Debug)]
pub struct ServerMetrics {
    requests_total: Counter<u64>,
    request_duration: Histogram<u64>,
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
                .u64_counter("server_requests")
                .with_description("total number of requests sent to fetch asset prices")
                .build(),
            request_duration: meter
                .u64_histogram("server_request_duration_milliseconds")
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

    pub fn record_requests_duration<T>(
        &self,
        elapsed_time: T,
        service_name: ServiceName,
        grpc_code: Code,
    ) where
        T: TryInto<u64>,
        T::Error: std::fmt::Display,
    {
        match elapsed_time.try_into() {
            Ok(elapsed_time) => self.request_duration.record(
                elapsed_time,
                &[
                    KeyValue::new("service_name", service_name.to_string()),
                    KeyValue::new("status", code_to_str(grpc_code)),
                ],
            ),
            Err(e) => warn!("failed to record request duration: {}", e),
        }
    }
}
