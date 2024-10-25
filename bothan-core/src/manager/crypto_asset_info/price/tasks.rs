use std::collections::{HashMap, HashSet, VecDeque};

use num_traits::Zero;
use rust_decimal::Decimal;
use tracing::{debug, info, warn};

use crate::manager::crypto_asset_info::price::cache::PriceCache;
use crate::manager::crypto_asset_info::price::error::{Error, MissingPrerequisiteError};
use crate::manager::crypto_asset_info::types::{
    PriceSignalComputationRecord, PriceSignalComputationRecords, PriceState, WorkerMap,
};
use crate::monitoring::records::{OperationRecord, SignalComputationRecord, SourceRecord};
use crate::registry::post_processor::PostProcess;
use crate::registry::processor::Process;
use crate::registry::signal::Signal;
use crate::registry::source::{OperationRoute, SourceQuery};
use crate::registry::{Registry, Valid};
use crate::store::ActiveSignalIDs;
use crate::worker::{AssetState, AssetWorker};

pub async fn get_signal_price_states<'a>(
    ids: Vec<String>,
    workers: &WorkerMap<'a>,
    registry: &Registry<Valid>,
    active_signal_ids: &ActiveSignalIDs,
    stale_cutoff: i64,
    records: &mut PriceSignalComputationRecords,
) -> Vec<PriceState> {
    let mut cache = PriceCache::new();

    let mut queue = VecDeque::from(ids.clone());
    while let Some(id) = queue.pop_front() {
        if cache.contains(&id) {
            continue;
        }

        match compute_signal_result(&id, workers, registry, stale_cutoff, &cache, records).await {
            Ok(price) => {
                info!("signal {}: {} ", id, price);
                cache.set_available(id, price);
            }
            Err(Error::PrerequisiteRequired(MissingPrerequisiteError {
                ids: prerequisite_ids,
            })) => {
                debug!(
                    "prerequisites required for signal {}: {:?}",
                    id, prerequisite_ids
                );
                queue.push_front(id);
                for prerequisite_id in prerequisite_ids {
                    queue.push_front(prerequisite_id)
                }
            }
            Err(Error::InvalidSignal) => {
                warn!("signal with id {} is not supported", id);
                cache.set_unsupported(id);
            }
            Err(Error::FailedToProcessSignal(e)) => {
                warn!("error while processing signal id {}: {}", id, e);
                cache.set_unavailable(id);
            }
            Err(Error::FailedToPostProcessSignal(e)) => {
                warn!("error while post processing signal id {}: {}", id, e);
                cache.set_unavailable(id);
            }
        }
    }

    ids.iter()
        .map(|id| {
            let is_active = active_signal_ids.contains(id);
            // This should never fail as all values of ids should be inserted into the cache
            let cache_value = cache.get(id).cloned().unwrap();
            match (is_active, cache_value) {
                (true, ps) => ps, // If the signal is active, return the cache value
                (false, PriceState::Unsupported) => PriceState::Unsupported, // If the signal is not active and is unsupported, return unsupported
                (false, _) => PriceState::Unavailable, // If the signal is not active and is available/unavailable, return unavailable
            }
        })
        .collect()
}

async fn compute_signal_result<'a>(
    id: &str,
    workers: &WorkerMap<'a>,
    registry: &Registry<Valid>,
    stale_cutoff: i64,
    cache: &PriceCache<String>,
    records: &mut PriceSignalComputationRecords,
) -> Result<Decimal, Error> {
    match registry.get(id) {
        Some(signal) => {
            let mut record = SignalComputationRecord::default();

            let source_results =
                compute_source_result(signal, workers, cache, stale_cutoff, &mut record).await?;

            let record_entry = records.entry(id.to_string()).or_insert(record);

            let process_signal_result = signal.processor.process(source_results);
            record_entry.process_result = Some(process_signal_result.clone());

            let processed_signal = process_signal_result?;

            let post_process_signal_result = signal
                .post_processors
                .iter()
                .try_fold(processed_signal, |acc, post| post.process(acc));
            record_entry.post_process_result = Some(post_process_signal_result.clone());

            Ok(post_process_signal_result?)
        }
        None => Err(Error::InvalidSignal),
    }
}

async fn compute_source_result<'a>(
    signal: &Signal,
    workers: &WorkerMap<'a>,
    cache: &PriceCache<String>,
    stale_cutoff: i64,
    record: &mut PriceSignalComputationRecord,
) -> Result<Vec<(String, Decimal)>, MissingPrerequisiteError> {
    // Create a temporary cache here as we don't want to write to the main record until we can
    // confirm that all prerequisites is settled
    let mut records_cache = HashMap::new();

    let mut source_values = Vec::with_capacity(signal.source_queries.len());
    let missing_prerequisites = HashSet::new();
    for source_query in &signal.source_queries {
        if let Some(worker) = workers.get(&source_query.source_id) {
            let source_value_opt = process_source_query(
                worker.as_ref(),
                source_query,
                stale_cutoff,
                cache,
                &mut records_cache,
            )
            .await?;

            if let Some(source_val) = source_value_opt {
                source_values.push(source_val)
            }
        }
    }

    if !missing_prerequisites.len().is_zero() {
        return Err(MissingPrerequisiteError::from(missing_prerequisites));
    }

    // If we don't need to go back and find any prerequisites, we can write to the records
    record.sources = records_cache;

    Ok(source_values)
}

async fn process_source_query<'a>(
    worker: &'a dyn AssetWorker,
    source_query: &SourceQuery,
    stale_cutoff: i64,
    cache: &PriceCache<String>,
    source_records: &mut HashMap<String, SourceRecord<Decimal>>,
) -> Result<Option<(String, Decimal)>, MissingPrerequisiteError> {
    let source_id = &source_query.source_id;
    let query_id = &source_query.query_id;
    match worker.get_asset(source_id).await {
        Ok(AssetState::Available(a)) if a.timestamp.ge(&stale_cutoff) => {
            // Create a record for the specific source
            let source_record = source_records
                .entry(source_id.clone())
                .or_insert(SourceRecord::new(query_id.clone(), a.price, vec![], None));

            // Calculate the source route
            compute_source_routes(&source_query.routes, a.price, cache, source_record)
                .map(|opt| opt.map(|price| (source_id.clone(), price)))
        }
        Ok(AssetState::Available(_)) => {
            warn!("asset state for {query_id} from {source_id} has timed out");
            Ok(None)
        }
        Ok(_) => {
            warn!("asset state for {query_id} from {source_id} is unavailable");
            Ok(None)
        }
        Err(_) => {
            warn!("error while querying source {source_id} for {query_id}");
            Ok(None)
        }
    }
}

fn compute_source_routes(
    routes: &Vec<OperationRoute>,
    start: Decimal,
    cache: &PriceCache<String>,
    record: &mut SourceRecord<Decimal>,
) -> Result<Option<Decimal>, MissingPrerequisiteError> {
    // Get all pre requisites
    let mut missing = Vec::with_capacity(routes.len());
    let mut values = Vec::with_capacity(routes.len());
    for route in routes {
        match cache.get(&route.signal_id) {
            Some(PriceState::Available(p)) => values.push(*p),
            None => missing.push(route.signal_id.clone()),
            Some(_) => return Ok(None), // If the value is not available, return None
        }
    }

    // If there are missing prerequisites, return an error
    if !missing.len().is_zero() {
        return Err(MissingPrerequisiteError::new(missing));
    }

    // Calculate the routed price
    let price = (0..routes.len()).try_fold(start, |acc, idx| {
        routes[idx].operation.execute(acc, values[idx])
    });

    // Modify operation record if necessary
    let op_records = routes
        .iter()
        .zip(values.iter())
        .map(|(r, v)| OperationRecord::new(r.signal_id.clone(), r.operation.clone(), *v))
        .collect::<Vec<OperationRecord>>();
    record.operations = op_records;

    Ok(price)
}
