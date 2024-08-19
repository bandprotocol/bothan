use crate::manager::crypto_asset_info::price::tasks::get_signal_price_states;
use crate::manager::crypto_asset_info::types::{PriceState, WorkerMap};
use crate::registry::{Registry, Valid};

pub(crate) mod cache;
pub(crate) mod tasks;

pub async fn get_prices<'a>(
    ids: Vec<String>,
    registry: &Registry<Valid>,
    workers: &WorkerMap<'a>,
    stale_threshold: i64,
) -> Vec<PriceState> {
    let current_time = chrono::Utc::now().timestamp();
    let stale_cutoff = current_time - stale_threshold;

    get_signal_price_states(ids.to_vec(), workers, registry, stale_cutoff).await
}
