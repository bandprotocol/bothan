//! Bothan core telemetry registry module.
//!
//! Provides initialization for Prometheus/OpenTelemetry registry.

use opentelemetry::global;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::{MetricError, SdkMeterProvider};
use prometheus::Registry;

const SERVICE_NAME: &str = "bothan";

pub fn init_telemetry_registry() -> Result<Registry, MetricError> {
    let registry = Registry::new();

    let exporter = opentelemetry_prometheus::exporter()
        .with_namespace(SERVICE_NAME)
        .with_registry(registry.clone())
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
