use std::collections::{HashMap, HashSet};
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
    for (source, query_ids) in query_ids.drain() {
        match workers.get(&source) {
            Some(worker) => match worker.set_query_ids(query_ids).await {
                Ok(_) => info!("removed query ids from {} worker", source),
                Err(e) => error!("worker {} failed to remove query ids: {}", source, e),
            },
            None => info!("worker {} not found", source),
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
        let Some(signal) = registry.get(signal_id) else {
            return Err(SetActiveSignalError::MissingSignal(signal_id.clone()));
        };
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
