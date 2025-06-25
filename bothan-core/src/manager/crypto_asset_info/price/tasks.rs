//! Price computation tasks for the crypto asset info manager.
//!
//! This module contains the logic to compute the price states for a list of signal ids.
//! It retrieves the necessary data from the store, processes it according to the signal's
//! processor and post-processors, and handles any missing prerequisites by recursively
//! fetching the required data.

use std::collections::HashSet;
use std::time::Instant;

use bothan_lib::metrics::store::{Metrics, Operation, OperationStatus};
use bothan_lib::registry::signal::Signal;
use bothan_lib::registry::source::{OperationRoute, SourceQuery};
use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::types::AssetInfo;
use num_traits::Zero;
use rust_decimal::Decimal;
use tracing::{debug, info, warn};

use crate::manager::crypto_asset_info::price::cache::PriceCache;
use crate::manager::crypto_asset_info::price::error::{Error, MissingPrerequisiteError};
use crate::manager::crypto_asset_info::types::{PriceSignalComputationRecord, PriceState};
use crate::monitoring::types::{OperationRecord, ProcessRecord, SourceRecord};

// TODO: Allow records to be Option<T>
/// Computes the price states for a list of signal ids.
pub async fn get_signal_price_states<S: Store>(
    ids: Vec<String>,
    store: &S,
    registry: &Registry<Valid>,
    stale_cutoff: i64,
    records: &mut Vec<PriceSignalComputationRecord>,
    metrics: &Metrics,
) -> Vec<PriceState> {
    let mut cache = PriceCache::new();

    let mut queue = ids.clone();
    while let Some(id) = queue.pop() {
        if cache.contains(&id) {
            continue;
        }

        let mut record = PriceSignalComputationRecord::new(id.clone());
        match compute_signal_result(
            &id,
            store,
            registry,
            stale_cutoff,
            &cache,
            &mut record,
            metrics,
        )
        .await
        {
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
                queue.push(id);
                for prerequisite_id in prerequisite_ids {
                    queue.push(prerequisite_id)
                }
                continue;
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
        records.push(record);
    }

    ids.into_iter()
        .map(|id| cache.get(&id).cloned().unwrap()) // This should never fail as all values of ids should be inserted into the cache
        .collect()
}

async fn compute_signal_result<S: Store>(
    id: &str,
    store: &S,
    registry: &Registry<Valid>,
    stale_cutoff: i64,
    cache: &PriceCache<String>,
    record: &mut PriceSignalComputationRecord,
    metrics: &Metrics,
) -> Result<Decimal, Error> {
    match registry.get(id) {
        Some(signal) => {
            let source_results =
                compute_source_result(signal, store, cache, stale_cutoff, record, metrics).await?;

            let process_signal_result = signal.processor.process(source_results);
            record.process_result = Some(ProcessRecord::new(
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
                        record.post_process_result = Some(post_process_records);
                        return Err(Error::FailedToPostProcessSignal(e));
                    }
                }
            }

            record.post_process_result = Some(post_process_records);

            Ok(processed_signal)
        }
        None => Err(Error::InvalidSignal),
    }
}

async fn compute_source_result<S: Store>(
    signal: &Signal,
    store: &S,
    cache: &PriceCache<String>,
    stale_cutoff: i64,
    record: &mut PriceSignalComputationRecord,
    metrics: &Metrics,
) -> Result<Vec<(String, Decimal)>, MissingPrerequisiteError> {
    // Create a temporary cache here as we don't want to write to the main record until we can
    // confirm that all prerequisites are settled
    let mut records_cache = Vec::new();

    let mut source_values = Vec::with_capacity(signal.source_queries.len());
    let mut missing_prerequisites = HashSet::new();
    for source_query in &signal.source_queries {
        let source_query_result = process_source_query(
            store,
            source_query,
            stale_cutoff,
            cache,
            &mut records_cache,
            metrics,
        )
        .await;

        match source_query_result {
            Ok(Some(source_val)) => source_values.push(source_val),
            Ok(None) => {
                // If the source value is None, we don't need to do anything
            }
            Err(MissingPrerequisiteError { ids }) => {
                // If we have a missing prerequisite, we need to add it to the list
                for id in ids {
                    missing_prerequisites.insert(id);
                }
            }
        }
    }

    if missing_prerequisites.is_empty() {
        // If we don't need to go back and find any prerequisites, we can write to the records
        record.sources = records_cache;

        Ok(source_values)
    } else {
        Err(MissingPrerequisiteError::from(missing_prerequisites))
    }
}

async fn process_source_query<S: Store>(
    store: &S,
    source_query: &SourceQuery,
    stale_cutoff: i64,
    cache: &PriceCache<String>,
    source_records: &mut Vec<SourceRecord<AssetInfo, Decimal>>,
    metrics: &Metrics,
) -> Result<Option<(String, Decimal)>, MissingPrerequisiteError> {
    let source_id = &source_query.source_id;
    let query_id = &source_query.query_id;

    // Create a record for the specific source
    let source_record = SourceRecord::new(source_id.clone(), query_id.clone(), None, vec![], None);
    source_records.push(source_record);
    // We can unwrap here because we just pushed the value, so it's guaranteed to be there
    let record = source_records.last_mut().unwrap();

    let start_time = Instant::now();
    match store.get_asset_info(source_id, query_id).await {
        Ok(Some(asset_info)) => {
            record.raw_source_value = Some(asset_info.clone());
            if asset_info.timestamp >= stale_cutoff {
                metrics.update_store_operation(
                    source_id.clone(),
                    start_time.elapsed().as_micros(),
                    Operation::GetAssetInfo,
                    OperationStatus::Success,
                );
                compute_source_routes(&source_query.routes, asset_info.price, cache, record)
                    .map(|opt| opt.map(|price| (source_id.clone(), price)))
            } else {
                warn!("asset state for {query_id} from {source_id} is stale");
                metrics.update_store_operation(
                    source_id.clone(),
                    start_time.elapsed().as_micros(),
                    Operation::GetAssetInfo,
                    OperationStatus::Stale,
                );
                Ok(None)
            }
        }
        Ok(None) => {
            warn!("asset state for {query_id} from {source_id} is not found");
            metrics.update_store_operation(
                source_id.clone(),
                start_time.elapsed().as_micros(),
                Operation::GetAssetInfo,
                OperationStatus::NotFound,
            );
            Ok(None)
        }
        Err(_) => {
            warn!("error while querying source {source_id} for {query_id}");
            metrics.update_store_operation(
                source_id.clone(),
                start_time.elapsed().as_micros(),
                Operation::GetAssetInfo,
                OperationStatus::Failed,
            );
            Ok(None)
        }
    }
}

fn compute_source_routes(
    routes: &Vec<OperationRoute>,
    init: Decimal,
    cache: &PriceCache<String>,
    record: &mut SourceRecord<AssetInfo, Decimal>,
) -> Result<Option<Decimal>, MissingPrerequisiteError> {
    // Get all prerequisites
    let mut missing = Vec::with_capacity(routes.len());
    let mut values = Vec::with_capacity(routes.len());
    for route in routes {
        match cache.get(&route.signal_id) {
            Some(PriceState::Available(p)) => values.push(*p),
            None => missing.push(route.signal_id.clone()),
            Some(_) => {
                warn!("signal {} is not available", route.signal_id);
                // If the value is not available, return None
                return Ok(None);
            }
        }
    }

    // If there are missing prerequisites, return an error
    if !missing.len().is_zero() {
        return Err(MissingPrerequisiteError::new(missing));
    }

    // Calculate the routed price
    let price = (0..routes.len()).try_fold(init, |acc, idx| {
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

    use bothan_lib::registry::processor::median::MedianProcessor;
    use bothan_lib::registry::processor::{ProcessError, Processor};
    use bothan_lib::registry::source::Operation;
    use bothan_lib::types::AssetInfo;
    use derive_more::Error;
    use num_traits::One;

    use super::*;
    use crate::monitoring::types::SignalComputationRecord;

    #[derive(Debug, Error)]
    struct MockError {}

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    #[derive(Default, Clone)]
    struct MockStore {
        expected_results: HashMap<String, AssetInfo>,
    }

    #[async_trait::async_trait]
    impl Store for MockStore {
        type Error = MockError;

        async fn set_registry(&self, _: Registry<Valid>, _: String) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn get_registry(&self) -> Result<Registry<Valid>, Self::Error> {
            Ok(Registry::default())
        }

        async fn get_registry_ipfs_hash(&self) -> Result<Option<String>, Self::Error> {
            Ok(None)
        }

        async fn get_asset_info(
            &self,
            prefix: &str,
            key: &str,
        ) -> Result<Option<AssetInfo>, Self::Error> {
            Ok(self
                .expected_results
                .get(&format!("{prefix}|{key}"))
                .cloned())
        }

        async fn insert_asset_info(&self, _: &str, _: AssetInfo) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn insert_batch_asset_info(
            &self,
            _: &str,
            _: Vec<AssetInfo>,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl MockStore {
        fn add_expected_query(&mut self, prefix: &str, id: &str, asset_state: AssetInfo) {
            self.expected_results
                .insert(format!("{prefix}|{id}"), asset_state);
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

        let mut mock_store = MockStore::default();
        mock_store.add_expected_query(
            "binance",
            "btcusdt",
            AssetInfo::new("btcusdt".to_string(), Decimal::new(69000, 0), 11000),
        );

        mock_store.add_expected_query(
            "coingecko",
            "bitcoin",
            AssetInfo::new("bitcoin".to_string(), Decimal::new(70000, 0), 11001),
        );
        mock_store.add_expected_query(
            "coingecko",
            "tether",
            AssetInfo::new("tether".to_string(), Decimal::one(), 11002),
        );

        let registry = mock_registry();
        let stale_cutoff = 0;
        let mut records = Vec::new();
        let metrics = Metrics::new();

        let res = get_signal_price_states(
            ids,
            &mock_store,
            &registry,
            stale_cutoff,
            &mut records,
            &metrics,
        )
        .await;

        let expected_res = vec![
            PriceState::Available(Decimal::new(69500, 0)),
            PriceState::Available(Decimal::one()),
        ];
        let expected_records = [
            SignalComputationRecord {
                signal_id: "CS:USDT-USD".to_string(),
                sources: vec![SourceRecord {
                    source_id: "coingecko".to_string(),
                    query_id: "tether".to_string(),
                    raw_source_value: Some(AssetInfo {
                        id: "tether".to_string(),
                        price: Decimal::from(1),
                        timestamp: 11002,
                    }),
                    operations: vec![],
                    final_value: Some(Decimal::from(1)),
                }],
                process_result: Some(ProcessRecord {
                    function: "median".to_string(),
                    result: Ok(Decimal::from(1)),
                }),
                post_process_result: Some(vec![]),
            },
            SignalComputationRecord {
                signal_id: "CS:BTC-USD".to_string(),
                sources: vec![
                    SourceRecord {
                        source_id: "binance".to_string(),
                        query_id: "btcusdt".to_string(),
                        raw_source_value: Some(AssetInfo {
                            id: "btcusdt".to_string(),
                            price: Decimal::from(69000),
                            timestamp: 11000,
                        }),
                        operations: vec![OperationRecord {
                            signal_id: "CS:USDT-USD".to_string(),
                            operation: Operation::Multiply,
                            value: Decimal::from(1),
                        }],
                        final_value: Some(Decimal::from(69000)),
                    },
                    SourceRecord {
                        source_id: "coingecko".to_string(),
                        query_id: "bitcoin".to_string(),
                        raw_source_value: Some(AssetInfo {
                            id: "bitcoin".to_string(),
                            price: Decimal::from(70000),
                            timestamp: 11001,
                        }),
                        operations: vec![],
                        final_value: Some(Decimal::from(70000)),
                    },
                ],
                process_result: Some(ProcessRecord {
                    function: "median".to_string(),
                    result: Ok(Decimal::from(69500)),
                }),
                post_process_result: Some(vec![]),
            },
        ];
        assert_eq!(res, expected_res);
        assert_eq!(records, expected_records);
    }

    #[tokio::test]
    async fn test_get_signal_price_states_with_unavailable() {
        let ids = vec!["CS:BTC-USD".to_string(), "CS:USDT-USD".to_string()];
        let mock_store = MockStore::default();
        let registry = mock_registry();
        let stale_cutoff = 0;
        let mut records = Vec::new();
        let metrics = Metrics::new();
        let res = get_signal_price_states(
            ids,
            &mock_store,
            &registry,
            stale_cutoff,
            &mut records,
            &metrics,
        )
        .await;

        let expected_res = vec![PriceState::Unavailable, PriceState::Unavailable];
        let expected_records = vec![
            SignalComputationRecord {
                signal_id: "CS:USDT-USD".to_string(),
                sources: vec![SourceRecord {
                    source_id: "coingecko".to_string(),
                    query_id: "tether".to_string(),
                    raw_source_value: None,
                    operations: vec![],
                    final_value: None,
                }],
                process_result: Some(ProcessRecord {
                    function: "median".to_string(),
                    result: Err(ProcessError::new("Not enough sources to calculate median")),
                }),
                post_process_result: None,
            },
            SignalComputationRecord {
                signal_id: "CS:BTC-USD".to_string(),
                sources: vec![
                    SourceRecord {
                        source_id: "binance".to_string(),
                        query_id: "btcusdt".to_string(),
                        raw_source_value: None,
                        operations: vec![],
                        final_value: None,
                    },
                    SourceRecord {
                        source_id: "coingecko".to_string(),
                        query_id: "bitcoin".to_string(),
                        raw_source_value: None,
                        operations: vec![],
                        final_value: None,
                    },
                ],
                process_result: Some(ProcessRecord {
                    function: "median".to_string(),
                    result: Err(ProcessError::new("Not enough sources to calculate median")),
                }),
                post_process_result: None,
            },
        ];
        assert_eq!(res, expected_res);
        assert_eq!(records, expected_records);
    }

    #[tokio::test]
    async fn test_get_signal_price_states_with_unsupported() {
        let ids = vec![
            "CS:BTC-USD".to_string(),
            "CS:USDT-USD".to_string(),
            "CS:DNE-USD".to_string(),
        ];

        let mut mock_store = MockStore::default();
        mock_store.add_expected_query(
            "binance",
            "btcusdt",
            AssetInfo::new("btcusdt".to_string(), Decimal::new(69000, 0), 11000),
        );
        mock_store.add_expected_query(
            "coingecko",
            "bitcoin",
            AssetInfo::new("bitcoin".to_string(), Decimal::new(70000, 0), 11001),
        );
        mock_store.add_expected_query(
            "coingecko",
            "tether",
            AssetInfo::new("tether".to_string(), Decimal::one(), 11002),
        );

        let registry = mock_registry();
        let stale_cutoff = 10000;
        let mut records = Vec::new();
        let metrics = Metrics::new();

        let res = get_signal_price_states(
            ids,
            &mock_store,
            &registry,
            stale_cutoff,
            &mut records,
            &metrics,
        )
        .await;

        let expected_res = vec![
            PriceState::Available(Decimal::new(69500, 0)),
            PriceState::Available(Decimal::one()),
            PriceState::Unsupported,
        ];
        let expected_records = vec![
            SignalComputationRecord {
                signal_id: "CS:DNE-USD".to_string(),
                sources: vec![],
                process_result: None,
                post_process_result: None,
            },
            SignalComputationRecord {
                signal_id: "CS:USDT-USD".to_string(),
                sources: vec![SourceRecord {
                    source_id: "coingecko".to_string(),
                    query_id: "tether".to_string(),
                    raw_source_value: Some(AssetInfo {
                        id: "tether".to_string(),
                        price: Decimal::from(1),
                        timestamp: 11002,
                    }),
                    operations: vec![],
                    final_value: Some(Decimal::from(1)),
                }],
                process_result: Some(ProcessRecord {
                    function: "median".to_string(),
                    result: Ok(Decimal::from(1)),
                }),
                post_process_result: Some(vec![]),
            },
            SignalComputationRecord {
                signal_id: "CS:BTC-USD".to_string(),
                sources: vec![
                    SourceRecord {
                        source_id: "binance".to_string(),
                        query_id: "btcusdt".to_string(),
                        raw_source_value: Some(AssetInfo {
                            id: "btcusdt".to_string(),
                            price: Decimal::from(69000),
                            timestamp: 11000,
                        }),
                        operations: vec![OperationRecord {
                            signal_id: "CS:USDT-USD".to_string(),
                            operation: Operation::Multiply,
                            value: Decimal::from(1),
                        }],
                        final_value: Some(Decimal::from(69000)),
                    },
                    SourceRecord {
                        source_id: "coingecko".to_string(),
                        query_id: "bitcoin".to_string(),
                        raw_source_value: Some(AssetInfo {
                            id: "bitcoin".to_string(),
                            price: Decimal::from(70000),
                            timestamp: 11001,
                        }),
                        operations: vec![],
                        final_value: Some(Decimal::from(70000)),
                    },
                ],
                process_result: Some(ProcessRecord {
                    function: "median".to_string(),
                    result: Ok(Decimal::from(69500)),
                }),
                post_process_result: Some(vec![]),
            },
        ];
        assert_eq!(res, expected_res);
        assert_eq!(records, expected_records);
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

        let mut mock_store = MockStore::default();
        let expected_asset_info = AssetInfo::new("testusd".to_string(), Decimal::default(), 0);
        mock_store.add_expected_query("test-source", "testusd", expected_asset_info.clone());

        let cache = PriceCache::new();
        let stale_cutoff = 0;
        let mut record = PriceSignalComputationRecord::new("test".to_string());
        let metrics = Metrics::new();

        let res = compute_source_result(
            &signal,
            &mock_store,
            &cache,
            stale_cutoff,
            &mut record,
            &metrics,
        )
        .await;

        let expected_res = Ok(vec![("test-source".to_string(), Decimal::default())]);
        let expected_record = SignalComputationRecord {
            signal_id: "test".to_string(),
            sources: vec![SourceRecord::new(
                "test-source".to_string(),
                "testusd".to_string(),
                Some(expected_asset_info),
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

        let mut mock_store = MockStore::default();
        mock_store.add_expected_query(
            "test-source",
            "testusd",
            AssetInfo::new("testusd".to_string(), Decimal::default(), 0),
        );

        let cache = PriceCache::new();
        let stale_cutoff = 0;
        let mut record = PriceSignalComputationRecord::new("test".to_string());
        let expected_record = record.clone();
        let metrics = Metrics::new();

        let res = compute_source_result(
            &signal,
            &mock_store,
            &cache,
            stale_cutoff,
            &mut record,
            &metrics,
        )
        .await;

        let expected_res = Err(MissingPrerequisiteError::new(vec!["test2usd".to_string()]));
        assert_eq!(res, expected_res);
        // We expect no mutation to the record here on missing prerequisite error here
        assert_eq!(record, expected_record);
    }

    #[tokio::test]
    async fn test_compute_source_result_with_multiple_missing_prerequisite() {
        let source_queries = vec![
            SourceQuery::new(
                "test-source-a".to_string(),
                "test1usd".to_string(),
                vec![OperationRoute::new(
                    "CS:ONE-USD".to_string(),
                    Operation::Multiply,
                )],
            ),
            SourceQuery::new(
                "test-source-b".to_string(),
                "test3usd".to_string(),
                vec![OperationRoute::new(
                    "CS:TWO-USD".to_string(),
                    Operation::Multiply,
                )],
            ),
        ];
        let processor = Processor::Median(MedianProcessor::new(1));
        let signal = Signal::new(source_queries, processor, vec![]);

        let mut mock_store = MockStore::default();
        mock_store.add_expected_query(
            "test-source-a",
            "test1usd",
            AssetInfo::new("test1usd".to_string(), Decimal::default(), 0),
        );
        mock_store.add_expected_query(
            "test-source-b",
            "test3usd",
            AssetInfo::new("test3usd".to_string(), Decimal::default(), 0),
        );

        let cache = PriceCache::new();
        let stale_cutoff = 0;
        let mut record = PriceSignalComputationRecord::new("CS:TEST-USD".to_string());
        let expected_record = record.clone();
        let metrics = Metrics::new();

        let res = compute_source_result(
            &signal,
            &mock_store,
            &cache,
            stale_cutoff,
            &mut record,
            &metrics,
        )
        .await;

        assert!(
            res.as_ref()
                .err()
                .unwrap()
                .ids
                .contains(&"CS:TWO-USD".to_string())
        );
        assert!(res.err().unwrap().ids.contains(&"CS:ONE-USD".to_string()));
        // We expect no mutation to the record here on missing prerequisite error here
        assert_eq!(record, expected_record);
    }

    #[tokio::test]
    async fn test_process_source_query() {
        let id = "testusd".to_string();
        let mut mock_store = MockStore::default();
        let asset_info = AssetInfo::new(id.clone(), Decimal::new(1000, 0), 10);
        mock_store.add_expected_query("test-source", "testusd", asset_info.clone());

        let source_query = SourceQuery::new("test-source".to_string(), id.clone(), vec![]);
        let stale_cutoff = 5;
        let cache = PriceCache::new();
        let source_records = &mut vec![];
        let metrics = Metrics::new();

        let res = process_source_query(
            &mock_store,
            &source_query,
            stale_cutoff,
            &cache,
            source_records,
            &metrics,
        )
        .await;

        let expected_res = Ok(Some(("test-source".to_string(), Decimal::new(1000, 0))));
        let expected_source_records = vec![SourceRecord::new(
            "test-source".to_string(),
            "testusd".to_string(),
            Some(asset_info),
            vec![],
            Some(Decimal::new(1000, 0)),
        )];
        assert_eq!(res, expected_res);
        assert_eq!(source_records, &expected_source_records);
    }

    #[tokio::test]
    async fn test_process_source_query_with_timeout() {
        let id = "testusd".to_string();
        let asset_info = AssetInfo::new(id.clone(), Decimal::default(), 0);

        let mut mock_store = MockStore::default();
        mock_store.add_expected_query("test-source", "testusd", asset_info.clone());

        let source_query = SourceQuery::new("test-source".to_string(), id.clone(), vec![]);
        let stale_cutoff = 1000;
        let cache = PriceCache::new();
        let source_records = &mut vec![];
        let metrics = Metrics::new();

        let res = process_source_query(
            &mock_store,
            &source_query,
            stale_cutoff,
            &cache,
            source_records,
            &metrics,
        )
        .await;

        let expected = vec![SourceRecord::new(
            "test-source".to_string(),
            "testusd".to_string(),
            Some(asset_info),
            vec![],
            None,
        )];

        assert_eq!(res, Ok(None));
        assert_eq!(source_records, &expected);
    }

    #[tokio::test]
    async fn test_process_source_query_with_unsupported_asset_state() {
        let source_query =
            SourceQuery::new("test-source".to_string(), "testusd".to_string(), vec![]);
        let stale_cutoff = 1000;
        let cache = PriceCache::new();
        let source_records = &mut vec![];
        let metrics = Metrics::new();

        let res = process_source_query(
            &MockStore::default(),
            &source_query,
            stale_cutoff,
            &cache,
            source_records,
            &metrics,
        )
        .await;

        let expected = vec![SourceRecord::new(
            "test-source".to_string(),
            "testusd".to_string(),
            None,
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

        let init = Decimal::one();

        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(2));
        cache.set_available("B".to_string(), Decimal::from(5));
        cache.set_available("C".to_string(), Decimal::from(13));
        cache.set_available("D".to_string(), Decimal::from(89));

        let asset_info = AssetInfo::new("test".to_string(), Decimal::one(), 0);
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_info.clone()),
            vec![],
            None,
        );
        let res = compute_source_routes(&routes, init, &cache, &mut record);

        let expected_value = Some(Decimal::new(764, 1));
        let expected_record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_info),
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
        let init = Decimal::one();
        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(2));

        let asset_info = AssetInfo::new("test".to_string(), Decimal::one(), 0);
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_info),
            vec![],
            None,
        );
        let expected_record = record.clone();
        let res = compute_source_routes(&routes, init, &cache, &mut record);

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
        let init = Decimal::one();
        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(1337));
        cache.set_unavailable("B".to_string());

        let asset_info = AssetInfo::new("test".to_string(), Decimal::one(), 0);
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_info),
            vec![],
            None,
        );
        let expected_record = record.clone();
        let res = compute_source_routes(&routes, init, &cache, &mut record);

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
        let init = Decimal::one();
        let mut cache = PriceCache::new();
        cache.set_available("A".to_string(), Decimal::from(10710110));
        cache.set_unsupported("B".to_string());
        cache.set_available("C".to_string(), Decimal::from(10000));

        let asset_info = AssetInfo::new("test".to_string(), Decimal::one(), 0);
        let mut record = SourceRecord::new(
            "test-source".to_string(),
            "test".to_string(),
            Some(asset_info),
            vec![],
            None,
        );
        let expected_record = record.clone();
        let res = compute_source_routes(&routes, init, &cache, &mut record);

        assert_eq!(res, Ok(None));
        // we expect no mutation to the record on failure here
        assert_eq!(record, expected_record);
    }
}
