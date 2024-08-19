use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tracing::{error, info};

use crate::manager::crypto_asset_info::error::SetActiveSignalError;
use crate::registry::{Registry, Valid};
use crate::store::QueryIds;
use crate::worker::AssetWorker;

pub async fn add_worker_query_ids<'a>(
    workers: &HashMap<String, Arc<dyn AssetWorker + 'a>>,
    current_active_set: &QueryIds,
    new_active_set: &QueryIds,
    registry: &Registry<Valid>,
) -> Result<(), SetActiveSignalError> {
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
    registry: &Registry<Valid>,
) -> Result<(), SetActiveSignalError> {
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
    registry: &Registry<Valid>,
) -> Result<HashMap<String, Vec<String>>, SetActiveSignalError> {
    let mut query_ids: HashMap<String, Vec<String>> = HashMap::new();

    for signal_id in signal_ids.iter() {
        let signal = registry
            .get(signal_id)
            .ok_or(SetActiveSignalError::MissingSignal(signal_id.clone()))?;
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
    use crate::registry::Invalid;

    use super::*;

    fn mock_registry() -> Registry<Valid> {
        let registry_str = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"coingecko\",\"id\":\"tether\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        let registry = serde_json::from_str::<Registry<Invalid>>(registry_str).unwrap();
        registry.validate().unwrap()
    }

    #[test]
    fn test_get_source_batched_query_ids() {
        let registry = mock_registry();

        let signal_ids = vec!["CS:BTC-USD".to_string()];
        let diff = get_source_batched_query_ids(&signal_ids, &registry);
        let expected = HashMap::from_iter([
            ("binance".to_string(), vec!["btcusdt".to_string()]),
            ("coingecko".to_string(), vec!["bitcoin".to_string()]),
        ]);
        assert_eq!(diff.unwrap(), expected);
    }
}
