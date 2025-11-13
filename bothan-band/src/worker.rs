//! Band worker implementation.
//!
//! This module provides an implementation of the [`AssetWorker`] trait for interacting with
//! the Band REST API. It defines the [`Worker`], which is responsible for periodically
//! polling [`AssetInfo`](bothan_lib::types::AssetInfo) from Band REST API and storing it to a shared [`WorkerStore`].
//!
//! The worker is configurable via [`WorkerOpts`] and uses [`RestApiBuilder`] to construct
//! the API client.
//!
//! # The module provides:
//! - Polling of [`AssetInfo`](bothan_lib::types::AssetInfo) periodically in asynchronous task
//! - Ensures graceful cancellation by using a CancellationToken to signal shutdown and a DropGuard
//!   to automatically clean up resources when the worker is dropped
//! - Metrics collection for observability
//! - Configurable via polling interval and endpoint URL
//!
//! # Examples
//!
//! ```rust,no_run
//! use bothan_band::worker::Worker;
//! use bothan_band::WorkerOpts;
//! use bothan_lib::worker::AssetWorker;
//! use bothan_lib::store::Store;
//!
//! #[tokio::test]
//! async fn test<T: Store>(store: T) {
//!     let opts = WorkerOpts::new("band", "https://example.com");
//!     let ids = vec!["CS:BTC-USD".to_string(), "CS:ETH-USD".to_string()];
//!
//!     let worker = Worker::build(opts, &store, ids).await?;
//! }
//! ```

use bothan_lib::metrics::rest::Metrics;
use bothan_lib::store::{Store, WorkerStore};
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use bothan_lib::worker::rest::start_polling;
use tokio_util::sync::{CancellationToken, DropGuard};
use tracing::instrument::Instrument;
use tracing::{Level, span};

use crate::WorkerOpts;
use crate::api::RestApiBuilder;

pub mod opts;

/// Asset worker for fetching data from the Band REST API.
///
/// The `Worker` manages asynchronous polling for [`AssetInfo`](bothan_lib::types::AssetInfo)
/// and ensures resources are properly cleaned up when dropped.
pub struct Worker {
    // Name identifier for the worker.
    name: &'static str,
    // We keep this DropGuard to ensure that all internal processes
    // that the worker holds are dropped when the worker is dropped.
    _drop_guard: DropGuard,
}

#[async_trait::async_trait]
impl AssetWorker for Worker {
    type Opts = WorkerOpts;

    /// Returns the name identifier for the worker.
    fn name(&self) -> &'static str {
        self.name
    }

    /// Builds and starts the `BandWorker`.
    ///
    /// This method creates a Band REST API client, spawns an asynchronous polling task
    /// to periodically fetch asset data, and returns the running [`Worker`] instance.
    ///
    /// # Errors
    ///
    /// Returns an [`AssetWorkerError`](bothan_lib::worker::error::AssetWorkerError) if:
    /// - The API client fails to build due to invalid configuration
    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError> {
        let name: &str = opts.name();
        let api = RestApiBuilder::new(opts.url).build()?;
        let worker_store = WorkerStore::new(store, name);
        let token = CancellationToken::new();
        let metrics = Metrics::new(name);

        let span = span!(Level::ERROR, "source", name = name);
        tokio::spawn(
            start_polling(
                token.child_token(),
                opts.update_interval,
                api,
                worker_store,
                ids,
                metrics,
            )
            .instrument(span),
        );

        Ok(Worker {
            name,
            _drop_guard: token.drop_guard(),
        })
    }
}
