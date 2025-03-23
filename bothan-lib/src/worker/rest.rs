use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use tokio::select;
use tokio::time::error::Elapsed;
use tokio::time::{interval, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

use crate::metrics::rest::{RequestStatus, ResponseLatencyStatus, RestMetrics};
use crate::store::{Store, WorkerStore};
use crate::types::AssetInfo;

#[async_trait::async_trait]
pub trait AssetInfoProvider: Send + Sync {
    type Error: Display;

    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error>;
}

/// Starts polling asset information from a provider at the specified update interval. This function
/// will not return until the asset_info_provider is dropped.
/// Any errors that occur during the polling process will be logged.
pub async fn start_polling<S: Store, E: Display, P: AssetInfoProvider<Error = E>>(
    cancellation_token: CancellationToken,
    update_interval: Duration,
    provider: P,
    store: WorkerStore<S>,
    ids: Vec<String>,
    worker_name: &'static str,
    metrics: Arc<RestMetrics>,
) {
    if ids.is_empty() {
        debug!("no ids to poll");
        return;
    }
    let mut interval = interval(update_interval);

    loop {
        select! {
            _ = cancellation_token.cancelled() => break,
            _ = interval.tick() => {},
        }

        select! {
            _ = cancellation_token.cancelled() => break,
            r = handle_get_asset_info(&provider, &ids, update_interval, worker_name, metrics.clone()) => handle_polling_result(r, &store, worker_name, metrics.clone()).await,
        }
    }
}

async fn handle_get_asset_info<P, E>(
    provider: &P,
    ids: &Vec<String>,
    timeout_interval: Duration,
    worker_name: &'static str,
    metrics: Arc<RestMetrics>,
) -> Result<Result<Vec<AssetInfo>, E>, Elapsed>
where
    E: Display,
    P: AssetInfoProvider<Error = E>,
{
    let start_time = chrono::Utc::now().timestamp_millis();
    let result = timeout(timeout_interval, provider.get_asset_info(ids)).await;
    if result.is_ok() {
        metrics.record_response_latency(worker_name, start_time, ResponseLatencyStatus::Success);
    } else {
        metrics.record_response_latency(worker_name, start_time, ResponseLatencyStatus::Failed);
    }

    result
}

async fn handle_polling_result<S, E>(
    poll_result: Result<Result<Vec<AssetInfo>, E>, Elapsed>,
    store: &WorkerStore<S>,
    worker_name: &'static str,
    metrics: Arc<RestMetrics>,
) where
    S: Store,
    E: Display,
{
    match poll_result {
        Ok(Ok(asset_info)) => {
            metrics.increment_total_requests(worker_name, RequestStatus::Success);
            if let Err(e) = store.set_batch_asset_info(asset_info).await {
                error!("failed to update asset info with error: {e}");
            } else {
                debug!("asset info updated successfully");
            }
        }
        Ok(Err(e)) => {
            metrics.increment_total_requests(worker_name, RequestStatus::Failed);
            error!("failed to update asset info with error: {e}");
        }
        Err(_) => {
            metrics.increment_total_requests(worker_name, RequestStatus::Timeout);
            error!("updating interval exceeded timeout");
        }
    }
}
