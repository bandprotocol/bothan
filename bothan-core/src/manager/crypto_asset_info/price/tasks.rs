use std::collections::{HashSet, VecDeque};

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
use crate::worker::{AssetState, AssetWorker};

// TODO: Allow records to be Option<T>
pub async fn get_signal_price_states<'a>(
    ids: Vec<String>,
    workers: &WorkerMap<'a>,
    registry: &Registry<Valid>,
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
        .map(|id| cache.get(id).cloned().unwrap()) // This should never fail as all values of ids should be inserted into the cache
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

            let record_ref = records.push(id.to_string(), record);

            let process_signal_result = signal.processor.process(source_results);
            record_ref.process_result = Some(process_signal_result.clone());

            let processed_signal = process_signal_result?;

            let post_process_signal_result = signal
                .post_processors
                .iter()
                .try_fold(processed_signal, |acc, post| post.process(acc));
            record_ref.post_process_result = Some(post_process_signal_result.clone());

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
    // confirm that all prerequisites are settled
    let mut records_cache = Vec::new();

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
    source_records: &mut Vec<(String, SourceRecord<Decimal>)>,
) -> Result<Option<(String, Decimal)>, MissingPrerequisiteError> {
    let source_id = &source_query.source_id;
    let query_id = &source_query.query_id;
    match worker.get_asset(source_id).await {
        Ok(AssetState::Available(a)) if a.timestamp.ge(&stale_cutoff) => {
            // Create a record for the specific source
            source_records.push((
                source_id.clone(),
                SourceRecord::new(query_id.clone(), a.price, vec![], None),
            ));
            // We can unwrap here because we just pushed the value, so it's guaranteed to be there
            let (_, record) = source_records.last_mut().unwrap();

            // Calculate the source route
            compute_source_routes(&source_query.routes, a.price, cache, record)
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

    let op_records = routes
        .iter()
        .zip(values.iter())
        .map(|(r, v)| OperationRecord::new(r.signal_id.clone(), r.operation.clone(), *v))
        .collect::<Vec<OperationRecord>>();
    record.operations = op_records;

    Ok(price)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use num_traits::One;

    use crate::registry::processor::median::MedianProcessor;
    use crate::registry::processor::Processor;
    use crate::registry::source::Operation;
    use crate::registry::tests::valid_mock_registry;
    use crate::store::error::Error as StoreError;
    use crate::types::AssetInfo;
    use crate::worker::SetQueryIDError;

    use super::*;

    #[derive(Default)]
    struct MockWorker {
        value: Decimal,
        timestamp: i64,
    }

    #[async_trait::async_trait]
    impl AssetWorker for MockWorker {
        async fn get_asset(&self, id: &str) -> Result<AssetState, StoreError> {
            Ok(AssetState::Available(AssetInfo::new(
                id.to_string(),
                self.value,
                self.timestamp,
            )))
        }

        async fn set_query_ids(&self, _: Vec<String>) -> Result<(), SetQueryIDError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockUnavailableWorker {}

    #[async_trait::async_trait]
    impl AssetWorker for MockUnavailableWorker {
        async fn get_asset(&self, _: &str) -> Result<AssetState, StoreError> {
            Ok(AssetState::Unsupported)
        }

        async fn set_query_ids(&self, _: Vec<String>) -> Result<(), SetQueryIDError> {
            Ok(())
        }
    }

    fn mock_workers<'a>(ids: Vec<String>) -> WorkerMap<'a> {
        ids.into_iter()
            .map(|id| {
                (
                    id.clone(),
                    Arc::new(MockWorker::default()) as Arc<dyn AssetWorker>,
                )
            })
            .collect()
    }

    fn mock_unavailable_workers<'a>(ids: Vec<String>) -> WorkerMap<'a> {
        ids.into_iter()
            .map(|id| {
                (
                    id.clone(),
                    Arc::new(MockUnavailableWorker::default()) as Arc<dyn AssetWorker>,
                )
            })
            .collect()
    }

    #[tokio::test]
    async fn test_get_signal_price_states() {
        let ids = vec!["CS:BTC-USD".to_string(), "CS:USDT-USD".to_string()];
        let workers = mock_workers(vec!["binance".to_string(), "coingecko".to_string()]);
        let registry = valid_mock_registry().validate().unwrap();
        let stale_cutoff = 0;
        let mut records = PriceSignalComputationRecords::default();

        let res =
            get_signal_price_states(ids, &workers, &registry, stale_cutoff, &mut records).await;

        let expected_res = vec![
            PriceState::Available(Decimal::default()),
            PriceState::Available(Decimal::default()),
        ];
        assert_eq!(res, expected_res);
    }

    #[tokio::test]
    async fn test_get_signal_price_states_with_unavailable() {
        let ids = vec!["CS:BTC-USD".to_string(), "CS:USDT-USD".to_string()];
        let workers =
            mock_unavailable_workers(vec!["binance".to_string(), "coingecko".to_string()]);
        let registry = valid_mock_registry().validate().unwrap();
        let stale_cutoff = 0;
        let mut records = PriceSignalComputationRecords::default();

        let res =
            get_signal_price_states(ids, &workers, &registry, stale_cutoff, &mut records).await;

        let expected_res = vec![PriceState::Unavailable, PriceState::Unavailable];
        assert_eq!(res, expected_res);
    }

    #[tokio::test]
    async fn test_get_signal_price_states_with_unsupported() {
        let ids = vec![
            "CS:BTC-USD".to_string(),
            "CS:USDT-USD".to_string(),
            "CS:DNE-USD".to_string(),
        ];
        let workers = mock_workers(vec!["binance".to_string(), "coingecko".to_string()]);
        let registry = valid_mock_registry().validate().unwrap();
        let stale_cutoff = 0;
        let mut records = PriceSignalComputationRecords::default();

        let res =
            get_signal_price_states(ids, &workers, &registry, stale_cutoff, &mut records).await;

        let expected_res = vec![
            PriceState::Available(Decimal::default()),
            PriceState::Available(Decimal::default()),
            PriceState::Unsupported,
        ];
        assert_eq!(res, expected_res);
    }

    #[tokio::test]
    async fn test_compute_source_result() {
        let source_queries = vec![SourceQuery::new(
            "test-source".to_string(),
            "testusd".to_string(),
            vec![],
        )];
        let processor = Processor::Median(MedianProcessor::new(1));
        let signal = Signal::new(source_queries, processor, vec![]);
        let workers = mock_workers(vec!["test-source".to_string()]);
        let cache = PriceCache::new();
        let stale_cutoff = 0;
        let mut record = PriceSignalComputationRecord::default();

        let res = compute_source_result(&signal, &workers, &cache, stale_cutoff, &mut record).await;

        let expected_res = Ok(vec![("test-source".to_string(), Decimal::default())]);
        let expected_record = SignalComputationRecord {
            sources: vec![(
                "test-source".to_string(),
                SourceRecord::new("testusd".to_string(), Decimal::default(), vec![], None),
            )],
            process_result: None,
            post_process_result: None,
        };
        assert_eq!(res, expected_res);
        assert_eq!(record, expected_record);
    }

    #[tokio::test]
    async fn test_compute_source_result_with_missing_prerequisite() {
        let source_queries = vec![SourceQuery::new(
            "test-source".to_string(),
            "testusd".to_string(),
            vec![OperationRoute::new(
                "test2usd".to_string(),
                Operation::Multiply,
            )],
        )];
        let processor = Processor::Median(MedianProcessor::new(1));
        let signal = Signal::new(source_queries, processor, vec![]);
        let workers = mock_workers(vec!["test-source".to_string()]);
        let cache = PriceCache::new();
        let stale_cutoff = 0;
        let mut record = PriceSignalComputationRecord::default();
        let expected_record = record.clone();

        let res = compute_source_result(&signal, &workers, &cache, stale_cutoff, &mut record).await;

        let expected_res = Err(MissingPrerequisiteError::new(vec!["test2usd".to_string()]));
        assert_eq!(res, expected_res);
        // We expect no mutation to the record here on missing prerequisite error here
        assert_eq!(record, expected_record);
    }

    #[tokio::test]
    async fn test_process_source_query() {
        let worker = MockWorker::default();
        let source_query =
            SourceQuery::new("test-source".to_string(), "testusd".to_string(), vec![]);
        let stale_cutoff = 0;
        let cache = PriceCache::new();
        let source_records = &mut vec![];

        let res =
            process_source_query(&worker, &source_query, stale_cutoff, &cache, source_records)
                .await;

        let expected_res = Ok(Some(("test-source".to_string(), Decimal::default())));
        let expected_source_records = vec![(
            "test-source".to_string(),
            SourceRecord::new("testusd".to_string(), Decimal::default(), vec![], None),
        )];
        assert_eq!(res, expected_res);
        assert_eq!(source_records, &expected_source_records);
    }

    #[tokio::test]
    async fn test_process_source_query_with_timeout() {
        let worker = MockWorker::default();
        let source_query =
            SourceQuery::new("test-source".to_string(), "testusd".to_string(), vec![]);
        let stale_cutoff = 1000;
        let cache = PriceCache::new();
        let source_records = &mut vec![];

        let res =
            process_source_query(&worker, &source_query, stale_cutoff, &cache, source_records)
                .await;
        assert_eq!(res, Ok(None));
        assert_eq!(source_records, &vec![]);
    }

    #[tokio::test]
    async fn test_process_source_query_with_unavailable_asset_state() {
        let worker = MockUnavailableWorker::default();
        let source_query =
            SourceQuery::new("test-source".to_string(), "testusd".to_string(), vec![]);
        let stale_cutoff = 1000;
        let cache = PriceCache::new();
        let source_records = &mut vec![];

        let res =
            process_source_query(&worker, &source_query, stale_cutoff, &cache, source_records)
                .await;
        assert_eq!(res, Ok(None));
        assert_eq!(source_records, &vec![]);
    }

    #[test]
    fn test_compute_source_routes() {
        let routes = vec![
            OperationRoute::new("A", Operation::Multiply),
            OperationRoute::new("B", Operation::Divide),
            OperationRoute::new("C", Operation::Subtract),
            OperationRoute::new("D", Operation::Add),
        ];

        let start = Decimal::one();

        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(2));
        cache.set_available("B".to_string(), Decimal::from(5));
        cache.set_available("C".to_string(), Decimal::from(13));
        cache.set_available("D".to_string(), Decimal::from(89));

        let mut record = SourceRecord::new("test".to_string(), Decimal::one(), vec![], None);
        let res = compute_source_routes(&routes, start, &cache, &mut record);

        let expected_value = Some(Decimal::from_str_exact("76.4").unwrap());
        let expected_record = SourceRecord::new(
            "test".to_string(),
            Decimal::one(),
            vec![
                OperationRecord::new("A".to_string(), Operation::Multiply, Decimal::from(2)),
                OperationRecord::new("B".to_string(), Operation::Divide, Decimal::from(5)),
                OperationRecord::new("C".to_string(), Operation::Subtract, Decimal::from(13)),
                OperationRecord::new("D".to_string(), Operation::Add, Decimal::from(89)),
            ],
            None,
        );

        assert_eq!(res, Ok(expected_value));
        assert_eq!(record, expected_record);
    }

    #[test]
    fn test_compute_source_routes_with_missing_prerequisites() {
        let routes = vec![
            OperationRoute::new("A", Operation::Multiply),
            OperationRoute::new("B", Operation::Multiply),
        ];
        let start = Decimal::one();
        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(2));

        let mut record = SourceRecord::new("test".to_string(), Decimal::one(), vec![], None);
        let expected_record = record.clone();
        let res = compute_source_routes(&routes, start, &cache, &mut record);

        let expected_err = Err(MissingPrerequisiteError::new(vec!["B".to_string()]));

        assert_eq!(res, expected_err);
        // We expect that no mutation occurs in the case of missing prerequisite error
        assert_eq!(record, expected_record);
    }

    #[test]
    fn test_compute_source_routes_with_unavailable_prerequisites() {
        let routes = vec![
            OperationRoute::new("A", Operation::Multiply),
            OperationRoute::new("B", Operation::Multiply),
        ];
        let start = Decimal::one();
        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(1337));
        cache.set_unavailable("B".to_string());

        let mut record = SourceRecord::new("test".to_string(), Decimal::one(), vec![], None);
        let expected_record = record.clone();
        let res = compute_source_routes(&routes, start, &cache, &mut record);

        assert_eq!(res, Ok(None));
        // we expect no mutation to the record on failure here
        assert_eq!(record, expected_record);
    }

    #[test]
    fn test_compute_source_routes_with_unsupported_prerequisites() {
        let routes = vec![
            OperationRoute::new("A", Operation::Multiply),
            OperationRoute::new("B", Operation::Multiply),
            OperationRoute::new("C", Operation::Divide),
        ];
        let start = Decimal::one();
        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(10710110));
        cache.set_unavailable("B".to_string());
        cache.set_available("C".to_string(), Decimal::from(10000));

        let mut record = SourceRecord::new("test".to_string(), Decimal::one(), vec![], None);
        let expected_record = record.clone();
        let res = compute_source_routes(&routes, start, &cache, &mut record);

        assert_eq!(res, Ok(None));
        // we expect no mutation to the record on failure here
        assert_eq!(record, expected_record);
    }
}
