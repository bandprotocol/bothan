use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinSet;

use bothan_core::service::{Service as CoreService, ServiceResult};
use bothan_core::types::PriceData as CorePriceData;

use crate::manager::price_service::types::{
    ResultsStore, ServiceMap, SignalResultsStore, SourceResultsStore,
};
use crate::manager::price_service::utils::into_key;
use crate::proto::query::query::{PriceData, PriceStatus};
use crate::registry::source::Route;
use crate::registry::Registry;
use crate::tasks::error::Error;
use crate::tasks::signal_task::SignalTask;
use crate::tasks::Tasks;
use crate::utils::arc_mutex;

/// PriceServiceManager is used to manage price services.
///
/// ## Example
/// ```no_run
/// use std::sync::Arc;
/// use bothan_core::service::Service;
///
/// async fn main() {
///     let registry = Arc::new(HashMap::new());
///     let mut manager = PriceServiceManager::new(registry).unwrap();
///     let service = Box::new(MockService::new());
///     
///     manager.add_service("mock".to_string(), service).await;
/// }
/// ```
pub struct PriceServiceManager {
    service_map: Arc<Mutex<ServiceMap<Box<dyn CoreService>>>>,
    registry: Arc<Registry>,
    stale_threshold: u64,
}

impl PriceServiceManager {
    /// Creates a new `PriceServiceManager` given a registry.
    pub fn new(registry: Arc<Registry>, stale_threshold: u64) -> Result<Self, Error> {
        match Tasks::from_registry(&registry) {
            Ok(_) => Ok(PriceServiceManager {
                service_map: arc_mutex!(HashMap::new()),
                registry,
                stale_threshold,
            }),
            Err(e) => Err(e),
        }
    }

    /// Add a service with an assigned name to the service map.
    pub async fn add_service(&mut self, name: String, service: Box<dyn CoreService>) {
        self.service_map
            .lock()
            .await
            .insert(name, arc_mutex!(service));
    }

    /// Gets the [`PriceData`](crate::proto::query::query::PriceData) of the given signal ids.
    pub async fn get_prices(&mut self, ids: &[&str]) -> Vec<PriceData> {
        let current_time = chrono::Utc::now().timestamp();
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
        let available = filter_available_ids(signal_ids, &registry);
        match get_filtered_registry(available.as_slice(), &registry) {
            Some(filtered_registry) => match Tasks::from_registry(&filtered_registry) {
                Ok(tasks) => {
                    let map = &self.service_map;
                    let src_store = source_results_store.clone();
                    let sig_store = signal_results_store.clone();
                    handle_tasks(
                        tasks,
                        map,
                        src_store,
                        sig_store,
                        current_time,
                        self.stale_threshold,
                    )
                    .await
                }
                Err(_) => set_unavailable(available.as_slice(), signal_results_store.clone()).await,
            },
            None => set_unavailable(available.as_slice(), signal_results_store.clone()).await,
        };

        get_result_from_store(ids, signal_results_store.clone()).await
    }
}

fn filter_available_ids<'a>(signal_ids: Vec<&'a str>, registry: &Registry) -> Vec<&'a str> {
    signal_ids
        .into_iter()
        .filter(|id| registry.contains_key(&id.to_string()))
        .collect()
}

fn get_filtered_registry(signal_ids: &[&str], registry: &Registry) -> Option<Registry> {
    let mut queue = VecDeque::from_iter(signal_ids.iter().map(|s| s.to_string()));
    let mut seen = HashMap::new();

    while let Some(signal_id) = queue.pop_front() {
        if let Some(signal) = registry.get(&signal_id) {
            seen.insert(signal_id, signal.clone());
            for pid in &signal.prerequisites {
                if !seen.contains_key(pid) {
                    queue.push_back(pid.clone());
                }
            }
        } else {
            return None;
        }
    }

    Some(seen.into_iter().collect())
}

async fn store_source_data(
    service_name: &str,
    ids: &[&str],
    service_results: Vec<ServiceResult<CorePriceData>>,
    store: Arc<SourceResultsStore>,
    current_time: i64,
    stale_threshold: u64,
) {
    let results: Vec<(String, f64)> = ids
        .iter()
        .zip(service_results)
        .filter_map(|(id, service)| {
            service.ok().and_then(|pd| {
                let key = into_key(service_name, id);
                if (current_time - pd.timestamp as i64) < stale_threshold as i64 {
                    f64::from_str(pd.price.as_str())
                        .ok()
                        .map(|price| (key, price))
                } else {
                    None
                }
            })
        })
        .collect();

    store.set_batched(results).await;
}

async fn process_source_routes(
    start: f64,
    routes: &Vec<Route>,
    signal_result_store: &SignalResultsStore,
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

async fn process_signal_task(
    signal_task: &SignalTask,
    source_results_store: &SourceResultsStore,
    signal_results_store: &SignalResultsStore,
) -> Result<f64, PriceStatus> {
    let mut data = Vec::new();
    for source in &signal_task.signal().sources {
        let key = into_key(&source.source_id, &source.id);
        let saved_price = source_results_store.get(&key).await;
        if let Some(price) = saved_price {
            let routed = process_source_routes(price, &source.routes, signal_results_store).await;
            if let Some(routed_price) = routed {
                data.push(routed_price);
            }
        }
    }

    let prerequisites_data = signal_results_store
        .get_batched(signal_task.signal().prerequisites.as_slice())
        .await
        .into_iter()
        .map(|v| v?.ok())
        .collect::<Option<Vec<f64>>>();

    match prerequisites_data {
        Some(pre_req) => match signal_task.execute(data, pre_req) {
            Some(price) => Ok(price),
            None => Err(PriceStatus::Unavailable),
        },
        None => Err(PriceStatus::Unavailable),
    }
}

async fn handle_tasks(
    tasks: Tasks,
    service_map: &Mutex<ServiceMap<Box<dyn CoreService>>>,
    source_results_store: Arc<SourceResultsStore>,
    signal_results_store: Arc<SignalResultsStore>,
    current_time: i64,
    stale_threshold: u64,
) {
    // Run all source tasks
    let mut task_set = JoinSet::new();
    let mut locked_service_map = service_map.lock().await;
    tasks.source_tasks().iter().for_each(|task| {
        let cloned_task = task.clone();
        if let Some(service) = locked_service_map.get_mut(task.source_name()) {
            let cloned_service = service.clone();
            let cloned_source_store = source_results_store.clone();
            task_set.spawn(async move {
                let mut locked_service = cloned_service.lock().await;
                let results = cloned_task.get_prices(&mut locked_service).await;
                store_source_data(
                    cloned_task.source_name(),
                    cloned_task.source_ids().as_slice(),
                    results,
                    cloned_source_store,
                    current_time,
                    stale_threshold,
                )
                .await
            });
        }
    });

    while task_set.join_next().await.is_some() {}

    // Run all signal tasks sequentially by batch
    for batched_signal_task in tasks.batched_signal_tasks() {
        // Run all signal tasks in the batch in parallel
        let mut join_set = JoinSet::new();
        for signal_task in batched_signal_task.iter() {
            let cloned_signal_task = signal_task.clone();
            let cloned_source_store = source_results_store.clone();
            let cloned_signal_store = signal_results_store.clone();
            join_set.spawn(async move {
                let result = process_signal_task(
                    &cloned_signal_task,
                    &cloned_source_store,
                    &cloned_signal_store,
                )
                .await;
                cloned_signal_store
                    .set(cloned_signal_task.signal_id(), result)
                    .await;
            });
        }

        while join_set.join_next().await.is_some() {}
    }
}

async fn set_unavailable(ids: &[&str], store: Arc<SignalResultsStore>) {
    let results = ids
        .iter()
        .map(|id| (*id, Err(PriceStatus::Unavailable)))
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
                price_status: PriceStatus::Available.into(),
            },
            Some(Err(e)) => PriceData {
                signal_id: k.to_string(),
                price: "".to_string(),
                price_status: e.into(),
            },
            None => PriceData {
                signal_id: k.to_string(),
                price: "".to_string(),
                price_status: PriceStatus::Unsupported.into(),
            },
        })
        .collect()
}
