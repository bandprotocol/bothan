use crate::store::{Store, WorkerStore};
use crate::types::AssetInfo;
use std::fmt::Display;
use std::sync::Weak;
use std::time::Duration;
use tokio::time::{interval, timeout};
use tracing::{debug, error};

#[async_trait::async_trait]
pub trait AssetInfoProvider: Send + Sync {
    type Error: Display;

    async fn get_asset_info(&self, ids: &[String]) -> Result<Vec<AssetInfo>, Self::Error>;
}

pub async fn start_polling<S: Store, E: Display>(
    update_interval: Duration,
    asset_info_provider: Weak<dyn AssetInfoProvider<Error = E>>,
    store: WorkerStore<S>,
) {
    let mut interval = interval(update_interval);
    while let Some(provider) = asset_info_provider.upgrade() {
        interval.tick().await;

        let ids = match store.get_query_ids().await {
            Ok(ids) => ids,
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
                // save state
                if let Err(e) = store
                    .set_assets(ids.into_iter().zip(asset_info.into_iter()).collect())
                    .await
                {
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
