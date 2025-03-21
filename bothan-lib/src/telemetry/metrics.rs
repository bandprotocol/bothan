use opentelemetry::{global, KeyValue};

pub struct Metrics{}

impl Metrics {
    pub fn increment_get_prices_total_requests() {
        global::meter("server")
        .u64_counter("get_prices_total_requests")
        .with_description("total number of requests sent to fetch asset prices")
        .build().add(1, &[]);
    }

    pub fn update_get_prices_responses(start_time: i64, success: bool) {
        let elapsed_time = (chrono::Utc::now().timestamp_millis() - start_time) as u64; 
        let meter = global::meter("server");
        
        meter.u64_counter("get_prices_total_responses")
        .with_description("total number of responses received for asset price requests")
        .build().add(1, &[KeyValue::new("success", success)]);

        meter.u64_histogram("get_prices_response_time")
        .with_description("time taken to fetch asset prices")
        .with_unit("milliseconds")
        .build().record(elapsed_time, &[KeyValue::new("success", success)]);
    }

    pub fn increment_rest_get_asset_info_total(source: &'static str, status: &'static str) {
        global::meter(source)
        .u64_counter("rest_get_asset_info_total")
        .with_description("total number of get_asset_info requests")
        .build().add(1, &[KeyValue::new("status", status)]);
    }

    pub fn record_rest_get_asset_info_latency(source: &'static str, start_time: i64, status: &'static str){
        let elapsed_time = (chrono::Utc::now().timestamp_millis() - start_time) as u64; 

        global::meter(source)
        .u64_histogram("rest_get_asset_info_latency")
        .with_description("time taken to fetch asset info for each worker")
        .with_unit("milliseconds")
        .build().record(elapsed_time, &[KeyValue::new("status", status)]);
    }

    pub fn increment_source_activity_message_count(source: &'static str, status: &'static str) {
        global::meter(source)
        .u64_counter("source_activity_message_count")
        .with_description("total number of messages sent by the source to indicate whether the source is active or not")
        .build().add(1, &[KeyValue::new("messsage_type", status)]);
    }

    pub fn record_source_connection_time(source: &'static str, start_time: i64, status: &'static str) {
        let elapsed_time = (chrono::Utc::now().timestamp_millis() - start_time) as u64; 

        global::meter(source)
        .u64_histogram("source_connection_time")
        .with_description("time taken for worker to establish a websocket connection to the source.")
        .with_unit("milliseconds")
        .build().record(elapsed_time, &[KeyValue::new("status", status)]);
    }

}