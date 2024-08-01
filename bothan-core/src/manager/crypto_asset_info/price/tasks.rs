use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use itertools::Itertools;
use rust_decimal::Decimal;
use tracing::debug;

use crate::manager::crypto_asset_info::error::{
    MissingSignalError, SignalTaskError, SourceRoutingError,
};
use crate::manager::crypto_asset_info::price::utils::is_stale;
use crate::manager::crypto_asset_info::utils::into_key;
use crate::registry::source::{OperationRoute, SourceQuery};
use crate::registry::Registry;
use crate::tasks::signal_task::SignalTask;
use crate::tasks::source_task::SourceTask;
use crate::tasks::TaskSet;
use crate::types::AssetInfo;
use crate::worker::{AssetState, AssetWorker};

pub fn create_reduced_registry(
    signal_ids: Vec<String>,
    registry: &Registry,
) -> Result<Registry, MissingSignalError> {
    let mut queue = VecDeque::from(signal_ids);
    let mut reduced_registry = HashMap::new();

    while let Some(signal_id) = queue.pop_front() {
        let signal = registry.get(&signal_id).ok_or(MissingSignalError {
            signal_id: signal_id.to_owned(),
        })?;

        reduced_registry.insert(signal_id.clone(), signal.clone());

        // Add the prerequisite signal_ids to the queue
        signal
            .source_queries
            .iter()
            .flat_map(|s| s.routes.iter().map(|r| &r.signal_id))
            .dedup()
            .for_each(|id| {
                if !reduced_registry.contains_key(id) {
                    queue.push_back(id.clone());
                }
            });
    }

    Ok(reduced_registry)
}

pub async fn execute_task_set<'a>(
    task_set: TaskSet,
    workers: &HashMap<String, Arc<dyn AssetWorker + 'a>>,
    current_time: i64,
    stale_threshold: i64,
) -> Result<HashMap<String, Decimal>, MissingSignalError> {
    let (source_tasks, signal_tasks) = task_set.split();

    let source_result =
        execute_source_tasks(source_tasks, workers, current_time, stale_threshold).await;

    debug!("Processing signal tasks");
    let mut res = HashMap::<String, Decimal>::with_capacity(signal_tasks.len());
    for signal_task in signal_tasks {
        match execute_signal_task(signal_task, &source_result, &res) {
            Ok((signal_id, price)) => {
                debug!("Processed signal task with signal_id: {}", signal_id);
                res.insert(signal_id, price);
            }
            Err(e) => debug!("Error processing signal task with error: {}", e),
        }
    }

    Ok(res)
}

async fn execute_source_tasks<'a>(
    source_tasks: Vec<SourceTask>,
    workers: &HashMap<String, Arc<dyn AssetWorker + 'a>>,
    current_time: i64,
    stale_threshold: i64,
) -> HashMap<String, AssetInfo> {
    let results_size = source_tasks
        .iter()
        .fold(0, |acc, task| acc + task.source_ids().len());
    let mut results = HashMap::with_capacity(results_size);

    for source_task in source_tasks {
        let source_id = source_task.source_name().to_string();
        if let Some(worker) = workers.get(&source_id) {
            worker
                .get_assets(&source_task.source_ids())
                .await
                .into_iter()
                .filter_map(|status| match status {
                    AssetState::Available(info)
                        if is_stale(info.timestamp, current_time, stale_threshold) =>
                    {
                        Some(info)
                    }
                    _ => None,
                })
                .for_each(|info| {
                    let key = into_key(&source_id, &info.id);
                    results.insert(key, info);
                });
        }
    }

    results
}

fn execute_signal_task(
    signal_task: SignalTask,
    source_results: &HashMap<String, AssetInfo>,
    signal_results: &HashMap<String, Decimal>,
) -> Result<(String, Decimal), SignalTaskError> {
    let routed_source_prices = signal_task
        .signal()
        .source_queries
        .iter()
        .map(|s| compute_routed_source_price(s, source_results, signal_results))
        .collect::<Result<Vec<Decimal>, SourceRoutingError>>()?;

    let signal_id = signal_task.signal_id();
    let processed_price = signal_task.execute_processor(routed_source_prices)?;
    let post_processed_price = signal_task.execute_post_processors(processed_price)?;

    Ok((signal_id.to_string(), post_processed_price))
}

fn compute_routed_source_price(
    source: &SourceQuery,
    source_results: &HashMap<String, AssetInfo>,
    signal_results: &HashMap<String, Decimal>,
) -> Result<Decimal, SourceRoutingError> {
    let source_id = &source.source_id;
    let id = &source.query_id;
    let key = into_key(source_id, id);

    let asset_info = source_results
        .get(&key)
        .ok_or(SourceRoutingError::MissingSource(key.to_owned()))?;

    let values = source
        .routes
        .iter()
        .map(|r| signal_results.get(&r.signal_id).cloned())
        .collect::<Option<Vec<Decimal>>>()
        .ok_or(SourceRoutingError::IncompletePrerequisites(key))?;

    let routed_price = compute_source_routes(asset_info.price, &source.routes, values);
    Ok(routed_price)
}

fn compute_source_routes(
    start: Decimal,
    routes: &[OperationRoute],
    values: Vec<Decimal>,
) -> Decimal {
    (0..routes.len()).fold(start, |acc, idx| {
        routes[idx].operation.execute(acc, values[idx])
    })
}
