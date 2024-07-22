use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use anyhow::Context;
use tracing::debug;

use bothan_core::worker::AssetWorker;

use crate::manager::crypto_asset_info::price::tasks::{create_reduced_registry, execute_tasks};
use crate::manager::crypto_asset_info::price::utils::get_price_id;
use crate::proto::query::Price;
use crate::registry::Registry;
use crate::tasks::Tasks;

mod tasks;
mod utils;

const PRECISION: u32 = 9;

pub async fn get_prices(
    ids: Vec<String>,
    registry: &Registry,
    workers: &HashMap<String, Arc<dyn AssetWorker>>,
    stale_threshold: i64,
) -> anyhow::Result<Vec<Price>> {
    let current_time = chrono::Utc::now().timestamp();

    // Split the signals into those that exist and those that do not.
    // Signal that are not available will return UNSUPPORTED
    debug!("Processing {} signals", ids.len());
    let (supported, unsupported): (Vec<String>, Vec<String>) = ids
        .iter()
        .cloned()
        .partition(|id| registry.contains_key(id));

    debug!("Supported signals: {:?}", supported);
    debug!("Unsupported signals: {:?}", unsupported);

    let unsupported_ids = unsupported.into_iter().collect::<HashSet<String>>();
    let reduced_registry = create_reduced_registry(supported.clone(), registry)
        .with_context(|| format!("Failed to create registry with signals: {:?}", supported))?;

    let tasks = Tasks::try_from(reduced_registry)
        .with_context(|| "Failed to create tasks from registry")?;

    let available = execute_tasks(tasks, workers, current_time, stale_threshold)
        .await
        .with_context(|| "Failed to execute tasks")?;

    let prices = ids
        .into_iter()
        .map(|id| get_price_id(id, &available, &unsupported_ids))
        .collect::<Vec<Price>>();

    Ok(prices)
}
