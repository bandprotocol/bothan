use std::sync::Arc;
use opentelemetry::global;
use opentelemetry_sdk::{metrics::{MetricError, SdkMeterProvider}, Resource};
use prometheus::Registry;

const SERVICE_NAME: &str = "bothan";

pub fn init_telemetry_registry() -> Result<Arc<Registry>, MetricError> {
    let registry = Arc::new(Registry::new());

    let exporter = opentelemetry_prometheus::exporter()
        .with_namespace(SERVICE_NAME)
        .with_registry(registry.as_ref().clone())
        .build()?;

    let resource = Resource::builder().with_service_name(SERVICE_NAME).build();
    
    let provider = SdkMeterProvider::builder()
        .with_reader(exporter)
        .with_resource(resource)
        .build();

    // Set the global meter provider
    global::set_meter_provider(provider);

    Ok(registry)
}