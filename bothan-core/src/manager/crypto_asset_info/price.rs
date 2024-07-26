use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tracing::debug;

use crate::manager::crypto_asset_info::error::GetPriceError;
use crate::manager::crypto_asset_info::price::tasks::{create_reduced_registry, execute_tasks};
use crate::manager::crypto_asset_info::price::utils::get_price_state;
use crate::manager::crypto_asset_info::types::PriceState;
use crate::registry::Registry;
use crate::tasks::Tasks;
use crate::worker::AssetWorker;

mod tasks;
mod utils;

const PRECISION: u32 = 9;

pub async fn get_prices(
    ids: &[String],
    registry: &Registry,
    workers: &HashMap<String, Arc<dyn AssetWorker>>,
    stale_threshold: i64,
) -> Result<Vec<PriceState>, GetPriceError> {
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
        .map_err(|e| GetPriceError::RegistryCreation(e.to_string()))?;

    let tasks = Tasks::try_from(reduced_registry)
        .map_err(|e| GetPriceError::TaskCreation(e.to_string()))?;

    let available = execute_tasks(tasks, workers, current_time, stale_threshold)
        .await
        .map_err(|e| GetPriceError::TaskExecution(e.to_string()))?;

    let price_states = ids
        .iter()
        .map(|id| get_price_state(id, &available, &unsupported_ids))
        .collect::<Vec<PriceState>>();

    debug!("Prices: {:?}", price_states);

    Ok(price_states)
}
