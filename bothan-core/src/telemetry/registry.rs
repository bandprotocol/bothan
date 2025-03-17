use std::sync::Arc;
use opentelemetry::global;
use opentelemetry_sdk::metrics::{MetricError, SdkMeterProvider};
use prometheus::Registry;

pub fn init_telemetry_registry() -> Result<Arc<Registry>, MetricError> {
    let registry = Arc::new(Registry::new());

    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.as_ref().clone())
        .build()?;
    
    let provider = SdkMeterProvider::builder().with_reader(exporter).build();

    // Set the global meter provider
    global::set_meter_provider(provider);

    Ok(registry)
}