use serde::{Deserialize, Serialize};

use crate::api::types::DEFAULT_URL;

/// Options for configuring the `BybitWorkerBuilder`.
///
/// `BybitWorkerBuilderOpts` provides a way to specify custom settings for creating a `BybitWorker`.
/// This struct allows users to set optional parameters such as the WebSocket URL, which will be used during the construction of the `BybitWorker`.
///
/// # Examples
///
/// ```rust
/// use bothan_bybit::worker::opts::WorkerOpts;
///
/// let opts = WorkerOpts {
///     url: "wss://stream.bybit.com/v5/public/spot".to_string(),
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkerOpts {
    /// The base URL for the worker's connection. If not provided, a default URL will be used.
    #[serde(default = "default_url")]
    pub url: String,
}

/// Returns the default WebSocket URL for the Bybit API.
fn default_url() -> String {
    DEFAULT_URL.to_string()
}

impl Default for WorkerOpts {
    /// Creates a new `WorkerOpts` with default values.
    ///
    /// This method initializes the configuration with the default Bybit WebSocket URL.
    ///
    /// # Returns
    ///
    /// A [`WorkerOpts`] instance with default settings.
    fn default() -> Self {
        Self { url: default_url() }
    }
}
