//! Telemetry and metrics for Bothan core.
//!
//! Bothan core telemetry module.
//!
//! Provides telemetry server and registry initialization.

pub use server::spawn_server;
mod server;

pub use registry::init_telemetry_registry;
mod registry;
