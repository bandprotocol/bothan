//! Worker trait and implementations for forex asset sources.

pub mod opts;

use std::collections::HashMap;

use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::worker::AssetWorker;
use bothan_lib::worker::error::AssetWorkerError;
use derive_more::From;
use tracing::{error, info};

use crate::manager::asset_info::forex::worker::opts::ForexAssetWorkerOpts;
use crate::manager::asset_info::signal_ids::get_source_batched_query_ids;
use crate::manager::asset_info::types::AssetType;

#[derive(From)]
pub enum ForexAssetWorker {
    Band(bothan_band::Worker),
}

#[async_trait::async_trait]
impl AssetWorker for ForexAssetWorker {
    type Opts = ForexAssetWorkerOpts;

    fn name(&self) -> &'static str {
        match self {
            ForexAssetWorker::Band(w) => w.name(),
        }
    }

    async fn build<S: Store + 'static>(
        opts: Self::Opts,
        store: &S,
        ids: Vec<String>,
    ) -> Result<Self, AssetWorkerError> {
        Ok(match opts {
            ForexAssetWorkerOpts::Band(opts) => {
                ForexAssetWorker::from(bothan_band::Worker::build(opts, store, ids).await?)
            }
        })
    }
}

pub async fn build_workers<S: Store + 'static>(
    registry: &Registry<Valid>,
    opts: &HashMap<String, ForexAssetWorkerOpts>,
    store: S,
) -> Vec<ForexAssetWorker> {
    let mut workers = Vec::with_capacity(opts.len());
    for (source_id, query_id) in get_source_batched_query_ids(registry, AssetType::Forex).drain() {
        match opts.get(&source_id) {
            Some(opts) => {
                let ids = query_id.into_iter().collect();
                let builder_callable = ForexAssetWorker::build(opts.clone(), &store, ids);
                let worker = match builder_callable.await {
                    Ok(worker) => worker,
                    Err(e) => {
                        error!("failed to build worker {}: {}", source_id, e);
                        continue;
                    }
                };
                workers.push(worker);
            }
            None => info!("worker {} not activated", source_id),
        }
    }

    workers
}
