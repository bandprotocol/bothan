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

#[derive(Clone, Debug)]
pub struct ServerMetrics {
    get_prices_total_requests: Counter<u64>,
    get_prices_total_responses: Counter<u64>,
    get_prices_response_time: Histogram<u64>,
}

impl ServerMetrics {
    pub fn new() -> Self {
        let meter = global::meter("server");

        Self {
            get_prices_total_requests: meter
                .u64_counter("get_prices_total_requests")
                .with_description("total number of requests sent to fetch asset prices")
                .build(),
            get_prices_total_responses: meter
                .u64_counter("get_prices_total_responses")
                .with_description("total number of responses received for asset price requests")
                .build(),
            get_prices_response_time: meter
                .u64_histogram("get_prices_response_time")
                .with_description("time taken to fetch asset prices")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_get_prices_total_requests(&self) {
        self.get_prices_total_requests.add(1, &[]);
    }

    pub fn update_get_prices_responses(&self, elapsed_time: u64, grpc_code: Code) {
        self.get_prices_total_responses
            .add(1, &[KeyValue::new("success", code_to_str(grpc_code))]);
        self.get_prices_response_time.record(
            elapsed_time,
            &[KeyValue::new("status", code_to_str(grpc_code))],
        );
    }
}
