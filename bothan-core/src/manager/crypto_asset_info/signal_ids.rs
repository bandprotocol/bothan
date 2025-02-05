use crate::manager::crypto_asset_info::worker::CryptoAssetWorker;
use bothan_lib::registry::{Registry, Valid};
use bothan_lib::store::Store;
use bothan_lib::worker::AssetWorker;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{error, info};

pub async fn set_workers_query_ids<S: Store + 'static>(
    workers: &HashMap<String, CryptoAssetWorker<S>>,
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
    use bothan_lib::registry::Invalid;

    fn mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"coingecko\",\"id\":\"tether\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    #[test]
    fn test_get_source_batched_query_ids() {
        let registry = mock_registry().validate().unwrap();

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
