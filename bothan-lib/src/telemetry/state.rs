use once_cell::sync::OnceCell;
use opentelemetry_sdk::metrics::{MetricError, SdkMeterProvider};
use prometheus::{proto::{self}, Registry};

static GLOBAL_METER_PROVIDER: OnceCell<SdkMeterProvider> = OnceCell::new();

pub struct TelemetryState {
    registry: Registry,
}

impl TelemetryState {
    pub fn build() -> Result<Self, MetricError>  {
        let registry = Registry::new();

        let exporter  = opentelemetry_prometheus::exporter()
            .with_registry(registry.clone()) 
            .build()?;
        
        let provider = SdkMeterProvider::builder().with_reader(exporter).build();

        match GLOBAL_METER_PROVIDER.set(provider.clone()) {
            Ok(_) => provider,
            Err(e) => e
        };

        Ok(Self {
            registry: registry,
        })
    }

    pub fn gather(&self) -> Vec<proto::MetricFamily> {
        self.registry.gather()
    }
}
