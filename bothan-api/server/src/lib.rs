pub mod api;
pub mod config;
pub mod manager;
pub mod proto;
pub mod registry;
pub mod tasks;
pub mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
