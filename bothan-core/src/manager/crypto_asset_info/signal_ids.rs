use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use tracing::{error, info};

use crate::manager::crypto_asset_info::error::SetActiveSignalError;
use crate::registry::{Registry, Valid};
use crate::worker::AssetWorker;

pub async fn set_workers_query_ids<'a>(
    workers: &HashMap<String, Arc<dyn AssetWorker + 'a>>,
    new_active_signal_ids: &HashSet<String>,
    registry: &Registry<Valid>,
) -> Result<(), SetActiveSignalError> {
    let mut query_ids = get_source_batched_query_ids(
        &new_active_signal_ids
            .iter()
            .cloned()
            .collect::<Vec<String>>(),
        registry,
    )?;

    // Find diff between current worker query ids and new query ids
    for (source, mut query_ids) in query_ids.drain() {
        match workers.get(&source) {
            Some(worker) => match worker.set_query_ids(query_ids.drain().collect()).await {
                Ok(_) => info!("set query ids for {} worker", source),
                Err(e) => error!("failed to set query ids for {} worker: {}", source, e),
            },
            None => info!("worker {} not found", source),
        }
    }

    Ok(())
}

fn get_source_batched_query_ids(
    signal_ids: &[String],
    registry: &Registry<Valid>,
) -> Result<HashMap<String, HashSet<String>>, SetActiveSignalError> {
    let mut source_query_ids: HashMap<String, HashSet<String>> = HashMap::new();
    // Seen signal_ids
    let mut seen = HashSet::<String>::new();

    let mut queue = VecDeque::from_iter(signal_ids);
    while let Some(signal_id) = queue.pop_front() {
        if seen.contains(signal_id) {
            continue;
        }

        let signal = registry
            .get(signal_id)
            .ok_or(SetActiveSignalError::UnsupportedSignal(signal_id.clone()))?;

        for source in &signal.source_queries {
            source_query_ids
                .entry(source.source_id.clone())
                .or_default()
                .insert(source.query_id.clone());

            for route in &source.routes {
                if seen.contains(&route.signal_id) {
                    continue;
                }
                queue.push_front(&route.signal_id);
            }
        }

        seen.insert(signal_id.clone());
    }
    Ok(source_query_ids)
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
            (
                "binance".to_string(),
                HashSet::from(["btcusdt".to_string()]),
            ),
            (
                "coingecko".to_string(),
                HashSet::from(["bitcoin".to_string(), "tether".to_string()]),
            ),
        ]);
        assert_eq!(diff.unwrap(), expected);
    }
}
