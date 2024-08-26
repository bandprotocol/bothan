use std::collections::{HashSet, VecDeque};

use num_traits::Zero;
use rust_decimal::Decimal;
use thiserror::Error;
use tracing::{error, info, warn};

use crate::manager::crypto_asset_info::price::cache::PriceCache;
use crate::manager::crypto_asset_info::types::{PriceState, WorkerMap};
use crate::registry::post_processor::{PostProcess, PostProcessError};
use crate::registry::processor::{Process, ProcessError};
use crate::registry::signal::Signal;
use crate::registry::source::OperationRoute;
use crate::registry::{Registry, Valid};
use crate::store::ActiveSignalIDs;
use crate::worker::AssetState;

#[derive(Debug, Error)]
enum Error {
    #[error("Signal does not exist")]
    InvalidSignal,

    #[error("Prerequisites required: {0:?}")]
    PrerequisiteRequired(Vec<String>),

    #[error("Failed to process signal: {0}")]
    FailedToProcessSignal(#[from] ProcessError),

    #[error("Failed to post process signal: {0}")]
    FailedToProcessPostSignal(#[from] PostProcessError),
}

pub async fn get_signal_price_states<'a>(
    ids: Vec<String>,
    workers: &WorkerMap<'a>,
    registry: &Registry<Valid>,
    active_signal_ids: &ActiveSignalIDs,
    stale_cutoff: i64,
) -> Vec<PriceState> {
    let mut cache = PriceCache::new();

    let mut queue = VecDeque::from(ids.clone());
    while let Some(id) = queue.pop_front() {
        if cache.contains(&id) {
            continue;
        }

        match compute_signal_result(&id, workers, registry, stale_cutoff, &cache).await {
            Ok(price) => {
                info!("signal {}: {} ", id, price);
                cache.set_available(id, price);
            }
            Err(Error::PrerequisiteRequired(prereqs)) => {
                info!("prerequisites required for signal {}: {:?}", id, prereqs);
                queue.push_front(id);
                for prereq in prereqs {
                    queue.push_front(prereq)
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
            Err(Error::FailedToProcessPostSignal(e)) => {
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
) -> Result<Decimal, Error> {
    match registry.get(id) {
        Some(signal) => {
            let source_results =
                compute_source_result(signal, workers, cache, stale_cutoff).await?;
            let processed_signal = signal.processor.process(source_results)?;
            let post_processed_signal = signal
                .post_processors
                .iter()
                .try_fold(processed_signal, |acc, post| post.process(acc))?;

            Ok(post_processed_signal)
        }
        None => Err(Error::InvalidSignal),
    }
}

async fn compute_source_result<'a>(
    signal: &Signal,
    workers: &WorkerMap<'a>,
    cache: &PriceCache<String>,
    stale_cutoff: i64,
) -> Result<Vec<Decimal>, Error> {
    // Check if all prerequisites are available, if not add the missing ones to the queue
    // and continue to the next signal
    let mut source_results = Vec::with_capacity(signal.source_queries.len());
    let mut missing_pids = HashSet::new();
    for source_query in &signal.source_queries {
        let qid = &source_query.query_id;
        let sid = &source_query.source_id;
        if let Some(w) = workers.get(sid) {
            match w.get_asset(qid).await {
                Ok(AssetState::Available(a)) => {
                    if a.timestamp.ge(&stale_cutoff) {
                        match compute_source_routes(&source_query.routes, a.price, cache) {
                            Ok(Some(price)) => source_results.push(price),
                            Ok(None) => {} // If unable to calculate the price, ignore the source
                            Err(Error::PrerequisiteRequired(ids)) => missing_pids.extend(ids),
                            Err(e) => return Err(e),
                        }
                    } else {
                        warn!("asset {qid} from {sid} is stale");
                    }
                }
                Ok(_) => warn!("asset state for {qid} from {sid} is unavailable"),
                Err(_) => warn!("error while querying source {sid} for {qid}"),
            }
        }
    }

    if !missing_pids.len().is_zero() {
        return Err(Error::PrerequisiteRequired(
            missing_pids.into_iter().collect(),
        ));
    }

    Ok(source_results)
}

fn compute_source_routes(
    routes: &Vec<OperationRoute>,
    start: Decimal,
    cache: &PriceCache<String>,
) -> Result<Option<Decimal>, Error> {
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
        return Err(Error::PrerequisiteRequired(missing));
    }

    // Calculate the routed price
    let price = (0..routes.len()).try_fold(start, |acc, idx| {
        routes[idx].operation.execute(acc, values[idx])
    });
    Ok(price)
}
