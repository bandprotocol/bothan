use opentelemetry::metrics::{Counter, Histogram};
use opentelemetry::{KeyValue, global};

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

#[derive(Clone, Debug)]
pub struct RestMetrics {
    requests_total: Counter<u64>,
    requests_duration: Histogram<u64>,
}

impl Default for RestMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RestMetrics {
    pub fn new() -> Self {
        let meter = global::meter("rest_source");
        Self {
            requests_total: meter
                .u64_counter("rest_requests_total")
                .with_description("total number of get_asset_info requests")
                .build(),
            requests_duration: meter
                .u64_histogram("rest_requests_duration_milliseconds")
                .with_description("time taken to fetch asset info for each worker")
                .with_unit("milliseconds")
                .build(),
        }
    }

    pub fn increment_requests_total(&self, source: &'static str, status: RequestStatus) {
        self.requests_total.add(
            1,
            &[
                KeyValue::new("status", status.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }

    pub fn record_requests_duration(
        &self,
        source: &'static str,
        elapsed_time: u64,
        status: RequestStatus,
    ) {
        self.requests_duration.record(
            elapsed_time,
            &[
                KeyValue::new("status", status.as_str_name()),
                KeyValue::new("source", source),
            ],
        );
    }
}
