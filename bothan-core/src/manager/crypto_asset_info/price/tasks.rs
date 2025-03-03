use std::collections::{HashMap, HashSet, VecDeque};

use bothan_lib::registry::signal::Signal;
use bothan_lib::registry::source::{OperationRoute, SourceQuery};
use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::types::AssetState;
use bothan_lib::worker::AssetWorker;
use num_traits::Zero;
use rust_decimal::Decimal;
use tracing::{debug, info, warn};

use crate::manager::crypto_asset_info::price::cache::PriceCache;
use crate::manager::crypto_asset_info::price::error::{Error, MissingPrerequisiteError};
use crate::manager::crypto_asset_info::types::{PriceSignalComputationRecord, PriceState};
use crate::monitoring::types::{
    OperationRecord, ProcessRecord, SignalComputationRecord, SourceRecord,
};

// TODO: Allow records to be Option<T>
/// Computes the price states for a list of signal ids.
pub async fn get_signal_price_states<S, W>(
    ids: Vec<String>,
    workers: &HashMap<String, W>,
    registry: &Registry<Valid>,
    stale_cutoff: i64,
    records: &mut Vec<PriceSignalComputationRecord>,
) -> Vec<PriceState>
where
    S: Store + 'static,
    W: AssetWorker<S>,
{
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

async fn compute_signal_result<S, W>(
    id: &str,
    workers: &HashMap<String, W>,
    registry: &Registry<Valid>,
    stale_cutoff: i64,
    cache: &PriceCache<String>,
    records: &mut Vec<PriceSignalComputationRecord>,
) -> Result<Decimal, Error>
where
    S: Store + 'static,
    W: AssetWorker<S>,
{
    match registry.get(id) {
        Some(signal) => {
            let mut record = SignalComputationRecord::new(id.to_string());

            let source_results =
                compute_source_result(signal, workers, cache, stale_cutoff, &mut record).await?;

            records.push(record);
            // We can unwrap here because we just pushed the record, so it's guaranteed to be there
            let record_ref = records.last_mut().unwrap();

            let process_signal_result = signal.processor.process(source_results);
            record_ref.process_result = Some(ProcessRecord::new(
                signal.processor.name().to_string(),
                process_signal_result.clone(),
            ));

            let mut processed_signal = process_signal_result?;
            let mut post_process_records = Vec::with_capacity(signal.post_processors.len());
            for post_processor in &signal.post_processors {
                let post_process_signal_result = post_processor.post_process(processed_signal);

                post_process_records.push(ProcessRecord::new(
                    post_processor.name().to_string(),
                    post_process_signal_result.clone(),
                ));

                match post_process_signal_result {
                    Ok(post_processed) => {
                        processed_signal = post_processed;
                    }
                    Err(e) => {
                        record_ref.post_process_result = Some(post_process_records);
                        return Err(Error::FailedToPostProcessSignal(e));
                    }
                }
            }

            record_ref.post_process_result = Some(post_process_records);

            Ok(processed_signal)
        }
        None => Err(Error::InvalidSignal),
    }
}

async fn compute_source_result<S, W>(
    signal: &Signal,
    workers: &HashMap<String, W>,
    cache: &PriceCache<String>,
    stale_cutoff: i64,
    record: &mut PriceSignalComputationRecord,
) -> Result<Vec<(String, Decimal)>, MissingPrerequisiteError>
where
    S: Store + 'static,
    W: AssetWorker<S>,
{
    // Create a temporary cache here as we don't want to write to the main record until we can
    // confirm that all prerequisites are settled
    let mut records_cache = Vec::new();

    let mut source_values = Vec::with_capacity(signal.source_queries.len());
    let missing_prerequisites = HashSet::new();
    for source_query in &signal.source_queries {
        if let Some(worker) = workers.get(&source_query.source_id) {
            let source_value_opt = process_source_query(
                worker,
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

async fn process_source_query<S, W>(
    worker: &W,
    source_query: &SourceQuery,
    stale_cutoff: i64,
    cache: &PriceCache<String>,
    source_records: &mut Vec<SourceRecord<AssetState, Decimal>>,
) -> Result<Option<(String, Decimal)>, MissingPrerequisiteError>
where
    S: Store + 'static,
    W: AssetWorker<S>,
{
    let source_id = &source_query.source_id;
    let query_id = &source_query.query_id;

    // Create a record for the specific source
    let source_record = SourceRecord::new(source_id.clone(), query_id.clone(), None, vec![], None);
    source_records.push(source_record);
    // We can unwrap here because we just pushed the value, so it's guaranteed to be there
    let record = source_records.last_mut().unwrap();

    match worker.get_asset(query_id).await {
        Ok(asset_state) => {
            record.raw_source_value = Some(asset_state.clone());
            match asset_state {
                AssetState::Available(a) if a.timestamp >= stale_cutoff => {
                    // Calculate the source route
                    compute_source_routes(&source_query.routes, a.price, cache, record)
                        .map(|opt| opt.map(|price| (source_id.clone(), price)))
                }
                AssetState::Available(_) => {
                    warn!("asset state for {query_id} from {source_id} is stale");
                    Ok(None)
                }
                AssetState::Unsupported => {
                    warn!("asset state for {query_id} from {source_id} is unsupported");
                    Ok(None)
                }
                AssetState::Pending => {
                    info!("asset state for {query_id} from {source_id} is pending");
                    Ok(None)
                }
            }
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
    record: &mut SourceRecord<AssetState, Decimal>,
) -> Result<Option<Decimal>, MissingPrerequisiteError> {
    // Get all prerequisites
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
    record.final_value = price;

    Ok(price)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt;
    use std::marker::PhantomData;

    use bothan_lib::registry::processor::Processor;
    use bothan_lib::registry::processor::median::MedianProcessor;
    use bothan_lib::registry::source::Operation;
    use bothan_lib::types::AssetInfo;
    use bothan_lib::worker::error::AssetWorkerError;
    use derive_more::Error;
    use num_traits::One;

    use super::*;

    #[derive(Debug, Error)]
    struct MockError {}

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    #[derive(Clone)]
    struct MockStore {}

    #[async_trait::async_trait]
    impl Store for MockStore {
        type Error = MockError;

        async fn set_registry(&self, _: Registry<Valid>, _: String) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn get_registry(&self) -> Registry<Valid> {
            Registry::default()
        }

        async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error> {
            Ok(None)
        }

        async fn set_query_ids(&self, _: &str, _: HashSet<String>) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn get_query_ids(&self, _: &str) -> Result<Option<HashSet<String>>, Self::Error> {
            Ok(None)
        }

        async fn contains_query_id(&self, _: &str, _: &str) -> Result<bool, Self::Error> {
            Ok(false)
        }

        async fn get_asset_info(&self, _: &str, _: &str) -> Result<Option<AssetInfo>, Self::Error> {
            Ok(None)
        }

        async fn insert_asset_info(&self, _: &str, _: AssetInfo) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn insert_asset_infos(&self, _: &str, _: Vec<AssetInfo>) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockWorker<S: Store> {
        expected_results: HashMap<String, AssetState>,
        phantom_data: PhantomData<S>,
    }

    impl<S: Store> MockWorker<S> {
        fn add_expected_query(&mut self, id: String, asset_state: AssetState) {
            self.expected_results.insert(id, asset_state);
        }
    }

    #[async_trait::async_trait]
    impl<S: Store> AssetWorker<S> for MockWorker<S> {
        type Opts = ();

        fn name(&self) -> &'static str {
            "mock"
        }

        async fn build(_: Self::Opts, _: &S) -> Result<Self, AssetWorkerError> {
            Ok(MockWorker {
                expected_results: HashMap::new(),
                phantom_data: PhantomData,
            })
        }

        async fn get_asset(&self, id: &str) -> Result<AssetState, AssetWorkerError> {
            Ok(self
                .expected_results
                .get(id)
                .unwrap_or_else(|| panic!("unexpected id {} received", id))
                .clone())
        }

        async fn set_query_ids(&self, _: HashSet<String>) -> Result<(), AssetWorkerError> {
            Ok(())
        }
    }

    pub fn mock_registry() -> Registry<Valid> {
        let json_string = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"coingecko\",\"id\":\"tether\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        let registry = serde_json::from_str::<Registry>(json_string).unwrap();
        registry.validate().unwrap()
    }

    #[tokio::test]
    async fn test_get_signal_price_states() {
        let ids = vec!["CS:BTC-USD".to_string(), "CS:USDT-USD".to_string()];

        let mut binance_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        binance_worker.add_expected_query(
            "btcusdt".to_string(),
            AssetState::Available(AssetInfo::new(
                "btcusdt".to_string(),
                Decimal::new(69000, 0),
                11000,
            )),
        );

        let mut coingecko_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        coingecko_worker.add_expected_query(
            "bitcoin".to_string(),
            AssetState::Available(AssetInfo::new(
                "bitcoin".to_string(),
                Decimal::new(70000, 0),
                11001,
            )),
        );
        coingecko_worker.add_expected_query(
            "tether".to_string(),
            AssetState::Available(AssetInfo::new("tether".to_string(), Decimal::one(), 11002)),
        );

        let mut workers = HashMap::new();
        workers.insert("binance".to_string(), binance_worker);
        workers.insert("coingecko".to_string(), coingecko_worker);

        let registry = mock_registry();
        let stale_cutoff = 0;
        let mut records = Vec::new();

        let res =
            get_signal_price_states(ids, &workers, &registry, stale_cutoff, &mut records).await;

        let expected_res = vec![
            PriceState::Available(Decimal::new(69500, 0)),
            PriceState::Available(Decimal::one()),
        ];
        assert_eq!(res, expected_res);
    }

    #[tokio::test]
    async fn test_get_signal_price_states_with_unavailable() {
        let ids = vec!["CS:BTC-USD".to_string(), "CS:USDT-USD".to_string()];
        let mut binance_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        binance_worker.add_expected_query("btcusdt".to_string(), AssetState::Unsupported);

        let mut coingecko_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        coingecko_worker.add_expected_query("bitcoin".to_string(), AssetState::Unsupported);
        coingecko_worker.add_expected_query("tether".to_string(), AssetState::Unsupported);

        let mut workers = HashMap::new();
        workers.insert("binance".to_string(), binance_worker);
        workers.insert("coingecko".to_string(), coingecko_worker);

        let registry = mock_registry();
        let stale_cutoff = 0;
        let mut records = Vec::new();

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

        let mut binance_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        binance_worker.add_expected_query(
            "btcusdt".to_string(),
            AssetState::Available(AssetInfo::new(
                "btcusdt".to_string(),
                Decimal::new(69000, 0),
                11000,
            )),
        );

        let mut coingecko_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        coingecko_worker.add_expected_query(
            "bitcoin".to_string(),
            AssetState::Available(AssetInfo::new(
                "bitcoin".to_string(),
                Decimal::new(70000, 0),
                11001,
            )),
        );
        coingecko_worker.add_expected_query(
            "tether".to_string(),
            AssetState::Available(AssetInfo::new("tether".to_string(), Decimal::one(), 11002)),
        );

        let mut workers = HashMap::new();
        workers.insert("binance".to_string(), binance_worker);
        workers.insert("coingecko".to_string(), coingecko_worker);

        let registry = mock_registry();
        let stale_cutoff = 10000;
        let mut records = Vec::new();

        let res =
            get_signal_price_states(ids, &workers, &registry, stale_cutoff, &mut records).await;

        let expected_res = vec![
            PriceState::Available(Decimal::new(69500, 0)),
            PriceState::Available(Decimal::one()),
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

        let mut test_source_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        let asset_state =
            AssetState::Available(AssetInfo::new("testusd".to_string(), Decimal::default(), 0));
        test_source_worker.add_expected_query("testusd".to_string(), asset_state.clone());

        let mut workers = HashMap::new();
        workers.insert("test-source".to_string(), test_source_worker);

        let cache = PriceCache::new();
        let stale_cutoff = 0;
        let mut record = PriceSignalComputationRecord::new("test".to_string());

        let res = compute_source_result(&signal, &workers, &cache, stale_cutoff, &mut record).await;

        let expected_res = Ok(vec![("test-source".to_string(), Decimal::default())]);
        let expected_record = SignalComputationRecord {
            signal_id: "test".to_string(),
            sources: vec![SourceRecord::new(
                "test-source".to_string(),
                "testusd".to_string(),
                Some(asset_state),
                vec![],
                Some(Decimal::default()),
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

        let mut test_source_worker = MockWorker::build((), &MockStore {}).await.unwrap();
        test_source_worker.add_expected_query(
            "testusd".to_string(),
            AssetState::Available(AssetInfo::new("testusd".to_string(), Decimal::default(), 0)),
        );

        let mut workers = HashMap::new();
        workers.insert("test-source".to_string(), test_source_worker);

        let cache = PriceCache::new();
        let stale_cutoff = 0;
        let mut record = PriceSignalComputationRecord::new("test".to_string());
        let expected_record = record.clone();

        let res = compute_source_result(&signal, &workers, &cache, stale_cutoff, &mut record).await;

        let expected_res = Err(MissingPrerequisiteError::new(vec!["test2usd".to_string()]));
        assert_eq!(res, expected_res);
        // We expect no mutation to the record here on missing prerequisite error here
        assert_eq!(record, expected_record);
    }

    #[tokio::test]
    async fn test_process_source_query() {
        let mut worker = MockWorker::build((), &MockStore {}).await.unwrap();
        let id = "testusd".to_string();
        let asset_state =
            AssetState::Available(AssetInfo::new(id.clone(), Decimal::new(1000, 0), 10));
        worker.add_expected_query("testusd".to_string(), asset_state.clone());

        let source_query = SourceQuery::new("test-source".to_string(), id.clone(), vec![]);
        let stale_cutoff = 5;
        let cache = PriceCache::new();
        let source_records = &mut vec![];

        let res =
            process_source_query(&worker, &source_query, stale_cutoff, &cache, source_records)
                .await;

        let expected_res = Ok(Some(("test-source".to_string(), Decimal::new(1000, 0))));
        let expected_source_records = vec![SourceRecord::new(
            "test-source".to_string(),
            "testusd".to_string(),
            Some(asset_state),
            vec![],
            Some(Decimal::new(1000, 0)),
        )];
        assert_eq!(res, expected_res);
        assert_eq!(source_records, &expected_source_records);
    }

    #[tokio::test]
    async fn test_process_source_query_with_timeout() {
        let mut worker = MockWorker::build((), &MockStore {}).await.unwrap();
        let id = "testusd".to_string();
        let asset_info = AssetInfo::new(id.clone(), Decimal::default(), 0);
        worker.add_expected_query(
            "testusd".to_string(),
            AssetState::Available(asset_info.clone()),
        );

        let source_query = SourceQuery::new("test-source".to_string(), id.clone(), vec![]);
        let stale_cutoff = 1000;
        let cache = PriceCache::new();
        let source_records = &mut vec![];

        let res =
            process_source_query(&worker, &source_query, stale_cutoff, &cache, source_records)
                .await;

        let expected = vec![SourceRecord::new(
            "test-source".to_string(),
            "testusd".to_string(),
            Some(AssetState::Available(asset_info)),
            vec![],
            None,
        )];

        assert_eq!(res, Ok(None));
        assert_eq!(source_records, &expected);
    }

    #[tokio::test]
    async fn test_process_source_query_with_unsupported_asset_state() {
        let mut worker = MockWorker::build((), &MockStore {}).await.unwrap();
        worker.add_expected_query("testusd".to_string(), AssetState::Unsupported);

        let source_query =
            SourceQuery::new("test-source".to_string(), "testusd".to_string(), vec![]);
        let stale_cutoff = 1000;
        let cache = PriceCache::new();
        let source_records = &mut vec![];

        let res =
            process_source_query(&worker, &source_query, stale_cutoff, &cache, source_records)
                .await;

        let expected = vec![SourceRecord::new(
            "test-source".to_string(),
            "testusd".to_string(),
            Some(AssetState::Unsupported),
            vec![],
            None,
        )];

        assert_eq!(res, Ok(None));
        assert_eq!(source_records, &expected);
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

        let asset_state =
            AssetState::Available(AssetInfo::new("test".to_string(), Decimal::one(), 0));
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_state.clone()),
            vec![],
            None,
        );
        let res = compute_source_routes(&routes, start, &cache, &mut record);

        let expected_value = Some(Decimal::new(764, 1));
        let expected_record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_state),
            vec![
                OperationRecord::new("A".to_string(), Operation::Multiply, Decimal::from(2)),
                OperationRecord::new("B".to_string(), Operation::Divide, Decimal::from(5)),
                OperationRecord::new("C".to_string(), Operation::Subtract, Decimal::from(13)),
                OperationRecord::new("D".to_string(), Operation::Add, Decimal::from(89)),
            ],
            expected_value,
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

        let asset_state =
            AssetState::Available(AssetInfo::new("test".to_string(), Decimal::one(), 0));
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_state),
            vec![],
            None,
        );
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

        let asset_state =
            AssetState::Available(AssetInfo::new("test".to_string(), Decimal::one(), 0));
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_state),
            vec![],
            None,
        );
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
        cache.set_unsupported("B".to_string());
        cache.set_available("C".to_string(), Decimal::from(10000));

        let asset_state =
            AssetState::Available(AssetInfo::new("test".to_string(), Decimal::one(), 0));
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_state),
            vec![],
            None,
        );
        let expected_record = record.clone();
        let res = compute_source_routes(&routes, start, &cache, &mut record);

        assert_eq!(res, Ok(None));
        // we expect no mutation to the record on failure here
        assert_eq!(record, expected_record);
    }
}
