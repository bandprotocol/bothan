use opentelemetry::{global, metrics::{Counter, Histogram}, KeyValue};

pub enum RequestStatus {
    Success,
    Failed,
    Timeout,
}

impl RequestStatus {
    fn as_str_name(&self) -> &'static str {
        match self {
            RequestStatus::Success => "success",
            RequestStatus::Failed => "failed",
            RequestStatus::Timeout => "timeout",
        }
    }
}

pub enum ResponseLatencyStatus {
    Success,
    Failed,
}

impl ResponseLatencyStatus {
    fn as_str_name(&self) -> &'static str {
        match self {
            ResponseLatencyStatus::Success => "success",
            ResponseLatencyStatus::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct RestMetrics {
    total_requests: Counter<u64>,
    response_latency: Histogram<u64>,
}

impl RestMetrics {
    pub fn new() -> Self {
        let meter = global::meter("rest_source");
        Self {
            total_requests: meter
                .u64_counter("total_requests")
                .with_description("total number of get_asset_info requests")
                .build(),
            response_latency: meter
                .u64_histogram("response_latency")
                .with_description("time taken to fetch asset info for each worker")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_total_requests(&self, source: &'static str, status: RequestStatus) {
        self.total_requests.add(1, &[
            KeyValue::new("status", status.as_str_name()),
            KeyValue::new("source", source),
        ]);
    }

    pub fn record_response_latency(&self, source: &'static str, elapsed_time: u64, status: ResponseLatencyStatus) {
        self.response_latency.record(elapsed_time, &[
            KeyValue::new("status", status.as_str_name()),
            KeyValue::new("source", source),
        ]);
    }
}