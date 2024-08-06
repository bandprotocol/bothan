use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tracing::{error, info};

use crate::manager::crypto_asset_info::error::MissingSignalError;
use crate::registry::Registry;
use crate::worker::AssetWorker;

pub async fn add_worker_query_ids<'a>(
    workers: &HashMap<String, Arc<dyn AssetWorker + 'a>>,
    current_active_set: &HashSet<String>,
    new_active_set: &HashSet<String>,
    registry: &Registry,
) -> Result<(), MissingSignalError> {
    let ids_to_add = new_active_set
        .difference(current_active_set)
        .cloned()
        .collect::<Vec<String>>();

    let mut query_ids = get_source_batched_query_ids(ids_to_add.as_slice(), registry)?;
    for (source_id, query_ids) in query_ids.drain() {
        match workers.get(&source_id) {
            Some(worker) => match worker.add_query_ids(query_ids).await {
                Ok(_) => info!("Added query ids to {} worker", source_id),
                Err(e) => error!("Worker {} failed to add query ids: {}", source_id, e),
            },
            None => info!("Worker {} not found", source_id),
        }
    }

    Ok(())
}

pub async fn remove_worker_query_ids<'a>(
    workers: &HashMap<String, Arc<dyn AssetWorker + 'a>>,
    current_active_set: &HashSet<String>,
    new_active_set: &HashSet<String>,
    registry: &Registry,
) -> Result<(), MissingSignalError> {
    let ids_to_rem = current_active_set
        .difference(new_active_set)
        .cloned()
        .collect::<Vec<String>>();

    let mut query_ids = get_source_batched_query_ids(ids_to_rem.as_slice(), registry)?;
    for (source, ids) in query_ids.drain() {
        match workers.get(&source) {
            Some(worker) => match worker.remove_query_ids(ids).await {
                Ok(_) => info!("Removed query ids from {} worker", source),
                Err(e) => error!("Worker {} failed to remove query ids: {}", source, e),
            },
            None => info!("Worker {} not found", source),
        }
    }

    Ok(())
}

fn get_source_batched_query_ids(
    signal_ids: &[String],
    registry: &Registry,
) -> Result<HashMap<String, Vec<String>>, MissingSignalError> {
    let mut query_ids: HashMap<String, Vec<String>> = HashMap::new();

    for signal_id in signal_ids.iter() {
        let signal = registry.get(signal_id).ok_or(MissingSignalError {
            signal_id: signal_id.to_owned(),
        })?;
        for source in signal.source_queries.iter() {
            query_ids
                .entry(source.source_id.clone())
                .or_default()
                .push(source.query_id.clone());
        }
    }

    Ok(query_ids)
}

#[cfg(test)]
mod tests {
    use crate::registry::processor::median::MedianProcessor;
    use crate::registry::processor::Processor;
    use crate::registry::signal::Signal;
    use crate::registry::source::SourceQuery;

    use super::*;

    fn mock_registry() -> Registry {
        let registry = HashMap::from_iter([
            (
                "CS:BTC-USD".to_string(),
                Signal::new(
                    vec![
                        SourceQuery::new("binance", "btcusdt", vec![]),
                        SourceQuery::new("coingecko", "bitcoin", vec![]),
                        SourceQuery::new("coinmarketcap", "bitcoin", vec![]),
                    ],
                    Processor::Median(MedianProcessor::new(1)),
                    vec![],
                ),
            ),
            (
                "CS:ETH-USD".to_string(),
                Signal::new(
                    vec![
                        SourceQuery::new("binance", "ethusdt", vec![]),
                        SourceQuery::new("coingecko", "ethereum", vec![]),
                        SourceQuery::new("coinmarketcap", "ethereum", vec![]),
                    ],
                    Processor::Median(MedianProcessor::new(1)),
                    vec![],
                ),
            ),
        ]);

        registry
    }

    #[test]
    fn test_get_source_batched_query_ids() {
        let registry = mock_registry();

        let signal_ids = vec!["CS:ETH-USD".to_string()];
        let diff = get_source_batched_query_ids(&signal_ids, &registry);
        let expected = HashMap::from_iter([
            ("binance".to_string(), vec!["ethusdt".to_string()]),
            ("coinmarketcap".to_string(), vec!["ethereum".to_string()]),
            ("coingecko".to_string(), vec!["ethereum".to_string()]),
        ]);
        assert_eq!(diff.unwrap(), expected);
    }
}
