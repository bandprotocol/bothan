use std::fmt::Display;
use std::time::Duration;

use tokio::select;
use tokio::time::error::Elapsed;
use tokio::time::{interval, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::metrics::rest::{PollingResult, RestMetrics};
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
#[tracing::instrument(skip(cancellation_token, provider, store, ids))]
pub async fn start_polling<S: Store, E: Display, P: AssetInfoProvider<Error = E>>(
    cancellation_token: CancellationToken,
    update_interval: Duration,
    provider: P,
    store: WorkerStore<S>,
    ids: Vec<String>,
    metrics: RestMetrics,
) {
    if ids.is_empty() {
        debug!("no ids to poll");
        return;
    }
    let mut interval = interval(update_interval);

    loop {
        select! {
            _ = cancellation_token.cancelled() => {
                info!("polling: cancelled");
                break
            },
            _ = interval.tick() => info!("polling"),
        }

        select! {
            _ = cancellation_token.cancelled() => break,
            r = poll_with_timeout(&provider, &ids, update_interval, &metrics) => handle_polling_result(r, &store, &metrics).await,
        }
    }
}

async fn poll_with_timeout<P, E>(
    provider: &P,
    ids: &[String],
    timeout_interval: Duration,
    metrics: &RestMetrics,
) -> Result<Result<Vec<AssetInfo>, E>, Elapsed>
where
    E: Display,
    P: AssetInfoProvider<Error = E>,
{
    let start_time = chrono::Utc::now().timestamp_millis();
    let result = timeout(timeout_interval, provider.get_asset_info(ids)).await;
    let elapsed_time = chrono::Utc::now().timestamp_millis() - start_time;

    let polling_result = match result {
        Ok(Ok(_)) => PollingResult::Success,
        Ok(Err(_)) => PollingResult::Failed,
        Err(_) => PollingResult::Timeout,
    };

    metrics.record_polling_duration(elapsed_time, polling_result);

    result
}

async fn handle_polling_result<S, E>(
    poll_result: Result<Result<Vec<AssetInfo>, E>, Elapsed>,
    store: &WorkerStore<S>,
    metrics: &RestMetrics,
) where
    S: Store,
    E: Display,
{
    let polling_result = match poll_result {
        Ok(Ok(asset_info)) => {
            if let Err(e) = store.set_batch_asset_info(asset_info).await {
                error!("failed to update asset info with error: {e}");
            } else {
                info!("asset info updated successfully");
            }
            PollingResult::Success
        }
        Ok(Err(e)) => {
            error!("failed to update asset info with error: {e}");
            PollingResult::Failed
        }
        Err(_) => {
            error!("updating interval exceeded timeout");
            PollingResult::Timeout
        }
    };

    metrics.increment_polling_total(polling_result);
}
