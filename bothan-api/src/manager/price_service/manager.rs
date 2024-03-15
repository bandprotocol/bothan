use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;

use itertools::{Either, Itertools};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tracing::warn;

pub use crate::manager::price_service::service::Service;
use crate::manager::price_service::types::{ResultsStore, SignalResultsStore, SourceResultsStore};
use crate::manager::price_service::utils::into_key;
use crate::post_processor::{PostProcessing, PostProcessor};
use crate::processor::{Processing, Processor};
use crate::proto::query::query::{PriceData, PriceOption};
use crate::registry::source::Route;
use crate::registry::{Registry, Signal};
use crate::tasks::task::Task;
use crate::tasks::Tasks;
use crate::util::arc_mutex;

pub struct PriceServiceManager {
    service_map: Arc<Mutex<HashMap<String, Arc<Mutex<Service>>>>>,
    registry: Arc<Registry>,
}

impl PriceServiceManager {
    pub fn new(registry: Arc<Registry>) -> Self {
        PriceServiceManager {
            service_map: arc_mutex!(HashMap::new()),
            registry,
        }
    }

    pub async fn add_service(&mut self, name: String, service: Service) {
        self.service_map
            .lock()
            .await
            .insert(name, arc_mutex!(service));
    }

    pub async fn get_prices(&mut self, ids: &[&str]) -> Vec<PriceData> {
        let registry = self.registry.clone();

        // remove duplicates
        let signal_ids = ids
            .iter()
            .cloned()
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect::<Vec<&str>>();

        // results store
        let source_results_store = Arc::new(ResultsStore::new());
        let signal_results_store = Arc::new(ResultsStore::new());

        // Split the signals into those that exist and those that do not
        let (filtered_registry, unsupported_signal_ids) =
            partition_signals(signal_ids.as_slice(), &registry);

        // load results into cache for the signals that do not exist
        set_unsupported_results(unsupported_signal_ids, signal_results_store.clone()).await;

        // Generate tasks for signals that exists
        let tasks_result = Tasks::from_registry(&filtered_registry);

        // If unable to generate tasks, return results
        match tasks_result {
            Err(_) => get_result_from_store(ids, signal_results_store.clone()).await,
            Ok(tasks) => {
                for task in tasks.iter() {
                    // process the source requirements for the task and saves the data
                    process_task_and_store_source_data(
                        task,
                        &self.service_map,
                        source_results_store.clone(),
                    )
                    .await;

                    // process the retrieved data
                    let mut join_set = JoinSet::new();
                    for (signal_id, signal) in task.get_signals() {
                        let cloned_id = signal_id.clone();
                        let cloned_signal = signal.clone();
                        let cloned_source_store = source_results_store.clone();
                        let cloned_signal_store = signal_results_store.clone();
                        join_set.spawn(async move {
                            get_and_store_signal_id_result(
                                cloned_id,
                                &cloned_signal,
                                cloned_source_store,
                                cloned_signal_store,
                            )
                            .await
                        });
                    }

                    while join_set.join_next().await.is_some() {}
                }
                get_result_from_store(ids, signal_results_store.clone()).await
            }
        }
    }
}

fn partition_signals(signal_ids: &[&str], registry: &Registry) -> (Registry, Vec<String>) {
    let mut exists = HashMap::new();
    let mut dne = HashSet::new();
    signal_ids.iter().for_each(|id| {
        let signal_id = id.to_string();
        match registry.get(&signal_id) {
            Some(signal) => {
                let all_prerequisite_signals = signal
                    .prerequisites
                    .iter()
                    .map(|id| registry.get(id).map(|signal| (id.clone(), signal.clone())))
                    .collect::<Option<Vec<(String, Signal)>>>();
                match all_prerequisite_signals {
                    Some(pre_req_signals) => {
                        // Add the original symbol to the set
                        exists.entry(signal_id).or_insert(signal.clone());
                        // Add the prerequisites to the set
                        pre_req_signals
                            .into_iter()
                            .for_each(|(pid, pre_req_signal)| {
                                exists.entry(pid).or_insert(pre_req_signal.clone());
                            })
                    }
                    None => {
                        dne.insert(signal_id.to_string());
                    }
                }
            }
            None => {
                dne.insert(signal_id.to_string());
            }
        }
    });

    (exists, dne.into_iter().collect())
}

async fn set_unsupported_results(
    unsupported_signal_ids: Vec<String>,
    store: Arc<SignalResultsStore>,
) {
    let results = unsupported_signal_ids
        .into_iter()
        .map(|id| (id, Err(PriceOption::Unsupported)))
        .collect();
    store.set_batched(results).await;
}

async fn get_result_from_store(ids: &[&str], store: Arc<SignalResultsStore>) -> Vec<PriceData> {
    store
        .get_batched(ids)
        .await
        .into_iter()
        .zip(ids)
        .map(|(v, k)| match v {
            Some(Ok(price)) => PriceData {
                signal_id: k.to_string(),
                price: price.to_string(),
                price_option: PriceOption::Available.into(),
            },
            Some(Err(e)) => PriceData {
                signal_id: k.to_string(),
                price: "".to_string(),
                price_option: e.into(),
            },
            None => PriceData {
                signal_id: k.to_string(),
                price: "".to_string(),
                price_option: PriceOption::Unsupported.into(),
            },
        })
        .collect()
}

async fn get_and_store_source_data(
    service: &Mutex<Service>,
    service_name: String,
    ids: Vec<String>,
    source_results_store: Arc<SourceResultsStore>,
) {
    let str_ids = ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let mut locked = service.lock().await;
    let data = locked.get_price_data(str_ids.as_slice()).await;
    drop(locked);

    let results: Vec<(String, f64)> = ids
        .into_iter()
        .zip(data)
        .filter_map(|(id, service)| {
            service.ok().and_then(|pd| {
                let key = into_key(&service_name, &id);
                f64::from_str(pd.price.as_str())
                    .ok()
                    .map(|price| (key, price))
            })
        })
        .collect();

    source_results_store.set_batched(results).await;
}

async fn process_task_and_store_source_data(
    task: &Task,
    service_map: &Mutex<HashMap<String, Arc<Mutex<Service>>>>,
    source_results_store: Arc<SourceResultsStore>,
) {
    let mut join_set = JoinSet::new();
    for (service_name, ids) in task.get_source_tasks() {
        let mut locked = service_map.lock().await;
        let service = locked.get_mut(service_name);

        if let Some(service) = service {
            let cloned_service = service.clone();
            let cloned_service_name = service_name.clone();
            let cloned_ids = ids.clone();
            let cloned_store = source_results_store.clone();

            join_set.spawn(async move {
                get_and_store_source_data(
                    &cloned_service,
                    cloned_service_name,
                    cloned_ids,
                    cloned_store,
                )
                .await
            });
        } else {
            warn!("Service {} not found", service_name);
        }
    }

    while join_set.join_next().await.is_some() {}
}

fn post_process(
    processed_price: f64,
    post_processors: &[PostProcessor],
) -> Result<f64, PriceOption> {
    let result: Option<f64> = post_processors
        .iter()
        .try_fold(processed_price, |acc, post| post.process(acc).ok());

    result.ok_or(PriceOption::Unavailable)
}

async fn process_source_routes(
    start: f64,
    routes: &Vec<Route>,
    signal_result_store: Arc<SignalResultsStore>,
) -> Option<f64> {
    // Pre-store and compute the fold values
    let mut signal_values = HashMap::new();
    for route in routes {
        let signal_id = route.signal_id.clone();
        let signal_result = signal_result_store.get(&signal_id).await?;
        let price = signal_result.ok()?;

        signal_values.insert(signal_id, price);
    }

    routes.iter().try_fold(start, |acc, route| {
        let price = signal_values.get(&route.signal_id)?;
        Some(route.operation.execute(acc, *price))
    })
}

async fn process_signal_id_result(
    data: Vec<f64>,
    prerequisities: &[String],
    processor: &Processor,
    post_processor: &[PostProcessor],
    signal_results_cache: &ResultsStore<Result<f64, PriceOption>>,
) -> Result<f64, PriceOption> {
    let prerequisites_data = signal_results_cache
        .get_batched(prerequisities)
        .await
        .into_iter()
        .map(|v| v?.ok())
        .collect::<Option<Vec<f64>>>();

    match prerequisites_data {
        Some(pre_data) => match processor.process(data, pre_data) {
            Ok(processed_price) => post_process(processed_price, post_processor),
            Err(_) => Err(PriceOption::Unavailable),
        },
        None => Err(PriceOption::Unavailable),
    }
}

async fn get_and_store_signal_id_result(
    signal_id: String,
    signal: &Signal,
    source_results_store: Arc<SourceResultsStore>,
    signal_results_store: Arc<SignalResultsStore>,
) {
    // Get source data for processing
    let mut data = Vec::new();
    for source in &signal.sources {
        let key = into_key(&source.source_id, &source.id);
        let saved = source_results_store.get(&key).await;
        if let Some(start) = saved {
            if let Some(price) =
                process_source_routes(start, &source.routes, signal_results_store.clone()).await
            {
                data.push(price);
            }
        }
    }

    // Process and store source data
    let price_data = process_signal_id_result(
        data,
        &signal.prerequisites,
        &signal.processor,
        &signal.post_processors,
        &signal_results_store,
    )
    .await;
    signal_results_store.set(signal_id, price_data).await;
}
