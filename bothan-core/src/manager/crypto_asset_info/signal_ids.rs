use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use tracing::{error, info};

use crate::registry::{Registry, Valid};
use crate::worker::AssetWorker;

pub async fn set_workers_query_ids<'a>(
    workers: &HashMap<String, Arc<dyn AssetWorker + 'a>>,
    registry: &Registry<Valid>,
) {
    for (source, mut query_ids) in get_source_batched_query_ids(registry).drain() {
        match workers.get(&source) {
            Some(worker) => match worker.set_query_ids(query_ids.drain().collect()).await {
                Ok(_) => info!("set query ids for {} worker", source),
                Err(e) => error!("failed to set query ids for {} worker: {}", source, e),
            },
            None => info!("worker {} not found", source),
        }
    }
}

fn get_source_batched_query_ids(registry: &Registry<Valid>) -> HashMap<String, HashSet<String>> {
    let mut source_query_ids: HashMap<String, HashSet<String>> = HashMap::new();
    // Seen signal_ids
    let mut seen = HashSet::<String>::new();

    let mut queue = VecDeque::from_iter(registry.keys());
    while let Some(signal_id) = queue.pop_front() {
        if seen.contains(signal_id) {
            continue;
        }

        // We unwrap here because we know the signal_id exists in the registry
        let signal = registry.get(signal_id).unwrap();

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
    source_query_ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::tests::valid_mock_registry;

    #[test]
    fn test_get_source_batched_query_ids() {
        let registry = valid_mock_registry().validate().unwrap();

        let diff = get_source_batched_query_ids(&registry);
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
        assert_eq!(diff, expected);
    }
}
