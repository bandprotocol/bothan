use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use anyhow::anyhow;
use itertools::Itertools;
use log::warn;
use rust_decimal::Decimal;
use tokio::task::JoinSet;
use tracing::debug;

use bothan_core::types::AssetInfo;
use bothan_core::worker::{AssetStatus, AssetWorker};

use crate::manager::crypto_asset_info::price::utils::filter_stale_assets;
use crate::manager::crypto_asset_info::utils::into_key;
use crate::registry::source::{OperationRoute, SourceQuery};
use crate::registry::Registry;
use crate::tasks::signal_task::SignalTask;
use crate::tasks::source_task::SourceTask;
use crate::tasks::Tasks;

pub fn create_reduced_registry(
    signal_ids: Vec<String>,
    registry: &Registry,
) -> anyhow::Result<Registry> {
    let mut queue = VecDeque::from(signal_ids);
    let mut reduced_registry: Registry = HashMap::new();

    while let Some(signal_id) = queue.pop_front() {
        let signal = registry
            .get(&signal_id)
            .ok_or(anyhow!("Missing signal with id: {}", signal_id))?;

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

pub async fn execute_tasks(
    tasks: Tasks,
    workers: &HashMap<String, Arc<dyn AssetWorker>>,
    current_time: i64,
    stale_threshold: i64,
) -> anyhow::Result<HashMap<String, Decimal>> {
    let (source_tasks, signal_tasks) = tasks.take_tasks();

    let results_size = source_tasks
        .iter()
        .fold(0, |acc, task| acc + task.source_ids().len());

    let join_set = start_source_tasks(source_tasks, workers);
    let source_result =
        process_source_tasks_results(join_set, current_time, stale_threshold, results_size).await;

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

fn start_source_tasks(
    tasks: Vec<SourceTask>,
    workers: &HashMap<String, Arc<dyn AssetWorker>>,
) -> JoinSet<(String, Vec<AssetStatus>)> {
    let mut join_set = JoinSet::new();
    for task in tasks {
        let source_id = task.source_name().to_string();
        if let Some(worker) = workers.get(&source_id).cloned() {
            join_set.spawn(async move { (source_id, worker.get_assets(&task.source_ids()).await) });
        }
    }

    join_set
}

async fn process_source_tasks_results(
    mut join_set: JoinSet<(String, Vec<AssetStatus>)>,
    current_time: i64,
    stale_threshold: i64,
    results_size: usize,
) -> HashMap<String, AssetInfo> {
    let mut results = HashMap::with_capacity(results_size);

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok((source_id, asset_statuses)) => asset_statuses
                .into_iter()
                .filter_map(|s| match s {
                    AssetStatus::Available(asset_info) => {
                        filter_stale_assets(asset_info, current_time, stale_threshold)
                    }
                    _ => None,
                })
                .for_each(|a| {
                    let key = into_key(&source_id, &a.id);
                    results.insert(key, a);
                }),
            Err(e) => warn!("Error fetching assets from source with error: {}", e),
        };
    }

    results
}

fn execute_signal_task(
    task: SignalTask,
    source_results: &HashMap<String, AssetInfo>,
    signal_results: &HashMap<String, Decimal>,
) -> anyhow::Result<(String, Decimal)> {
    let routed_source_prices = task
        .signal()
        .source_queries
        .iter()
        .map(|s| compute_routed_source_price(s, source_results, signal_results))
        .collect::<anyhow::Result<Vec<Decimal>>>()?;

    let signal_id = task.signal_id();
    let processed_price = task.execute_processor(routed_source_prices)?;
    let post_processed_price = task.execute_post_processors(processed_price)?;

    Ok((signal_id.to_string(), post_processed_price))
}

fn compute_routed_source_price(
    source: &SourceQuery,
    source_results: &HashMap<String, AssetInfo>,
    signal_results: &HashMap<String, Decimal>,
) -> anyhow::Result<Decimal> {
    let source_id = &source.source_id;
    let id = &source.query_id;
    let key = into_key(source_id, id);

    let asset_info = source_results
        .get(&key)
        .ok_or(anyhow!("no data found for {}", key))?;

    let values = source
        .routes
        .iter()
        .map(|r| signal_results.get(&r.signal_id).cloned())
        .collect::<Option<Vec<Decimal>>>()
        .ok_or(anyhow!("incomplete sources for {}", key))?;

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
