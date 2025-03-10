use std::fmt::Display;
use std::sync::Weak;
use std::time::Duration;

use tokio::time::{interval, timeout};
use tracing::{debug, error};

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
pub async fn start_polling<S: Store, E: Display>(
    update_interval: Duration,
    asset_info_provider: Weak<dyn AssetInfoProvider<Error = E>>,
    store: WorkerStore<S>,
) {
    let mut interval = interval(update_interval);
    while let Some(provider) = asset_info_provider.upgrade() {
        interval.tick().await;

        let ids = match store.get_query_ids().await {
            Ok(ids) => ids.into_iter().collect::<Vec<String>>(),
            Err(e) => {
                error!("failed to get query ids with error: {}", e);
                continue;
            }
        };

        if let Err(e) = store.get_query_ids().await {
            error!("failed to get query ids with error: {}", e);
            continue;
        }

        if ids.is_empty() {
            debug!("no ids to update, skipping update");
            continue;
        }

        match timeout(interval.period(), provider.get_asset_info(&ids)).await {
            Ok(Ok(asset_info)) => {
                if let Err(e) = store.set_asset_infos(asset_info).await {
                    error!("failed to update asset info with error: {e}");
                } else {
                    debug!("asset info updated successfully");
                }
            }
            Ok(Err(e)) => {
                error!("failed to update asset info with error: {e}");
            }
            Err(_) => {
                error!("updating interval exceeded timeout");
            }
        }
    }
}
