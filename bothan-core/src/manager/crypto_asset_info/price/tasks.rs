use std::collections::VecDeque;
use std::time::Duration;

use rust_decimal::Decimal;
use thiserror::Error;
use tokio::time::timeout;
use tracing::{error, info, warn};

use crate::manager::crypto_asset_info::price::cache::PriceCache;
use crate::manager::crypto_asset_info::types::{PriceState, WorkerMap};
use crate::registry::post_processor::{PostProcess, PostProcessError};
use crate::registry::processor::{Process, ProcessError};
use crate::registry::signal::Signal;
use crate::registry::source::OperationRoute;
use crate::registry::{Registry, Valid};
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
                info!("Signal {}: {} ", id, price);
                cache.set_available(id, price);
            }
            Err(Error::PrerequisiteRequired(prereqs)) => {
                info!("Prerequisites required for signal {}: {:?}", id, prereqs);
                queue.extend(prereqs);
                queue.push_back(id);
            }
            Err(Error::InvalidSignal) => {
                warn!("Signal with id {} is not supported", id);
                cache.set_unsupported(id);
            }
            Err(Error::FailedToProcessSignal(e)) => {
                warn!("Error while processing signal id {}: {}", id, e);
                cache.set_unavailable(id);
            }
            Err(Error::FailedToProcessPostSignal(e)) => {
                warn!("Error while post processing signal id {}: {}", id, e);
                cache.set_unavailable(id);
            }
        }
    }

    ids.iter()
        .map(|id| cache.get(id).cloned().unwrap()) // This should never fail
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
            // Check if all prerequisites are available, if not add the missing ones to the queue
            // and continue to the next signal
            let prereq_values = get_prerequisites(signal, cache)?;

            let mut source_results = Vec::with_capacity(signal.source_queries.len());
            for source_query in &signal.source_queries {
                if let Some(w) = workers.get(&source_query.source_id) {
                    // TODO move duration and add error
                    let aa = Duration::new(1, 0);
                    let future = async {
                        match w.get_asset(&source_query.query_id).await {
                            Ok(AssetState::Available(a)) => {
                                a.timestamp.ge(&stale_cutoff).then(|| {
                                    compute_source_routes(
                                        a.price,
                                        &source_query.routes,
                                        &prereq_values,
                                    )
                                })
                            }
                            _ => None,
                        }
                    };
                    match timeout(aa, future).await {
                        Ok(Some(price)) => source_results.push(price),
                        Ok(None) => error!("error while querying sources"),
                        Err(_) => error!("timed out while querying sources"),
                    }
                }
            }

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

// Note: The registry must not contain any cycles or this will loop forever
fn get_prerequisites(signal: &Signal, cache: &PriceCache<String>) -> Result<Vec<Decimal>, Error> {
    let pids = signal
        .source_queries
        .iter()
        .flat_map(|s| s.routes.iter().map(|r| r.signal_id.clone()))
        .collect::<Vec<String>>();

    let mut prereq_values = Vec::with_capacity(pids.len());
    let mut prereq_required = Vec::new();
    for pid in pids {
        match cache.get(&pid) {
            Some(PriceState::Available(p)) => prereq_values.push(*p),
            None => {
                // Add the prerequisite to the front of the queue
                prereq_required.push(pid.clone());
            }
            Some(_) => {}
        }
    }
    if !prereq_required.is_empty() {
        Err(Error::PrerequisiteRequired(prereq_required))
    } else {
        Ok(prereq_values)
    }
}

fn compute_source_routes(start: Decimal, routes: &[OperationRoute], values: &[Decimal]) -> Decimal {
    (0..routes.len()).fold(start, |acc, idx| {
        routes[idx].operation.execute(acc, values[idx])
    })
}
