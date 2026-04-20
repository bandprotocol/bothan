use std::collections::{HashMap, HashSet, VecDeque};

use bothan_lib::registry::{Registry, Valid};

// Returns a mapping of source_id to a set of query_ids that are batched together
// This is used to determine which queries should be batched together
// e.g. if we have a registry with the following entries:
// {
//     "CS:USDT-USD": {
//         "sources": [
//             {
//                 "source_id": "coingecko",
//                 "id": "tether",
//                 "routes": []
//             }
//         ],
//         "processor": {
//             "function": "median",
//             "params": {
//                 "min_source_count": 1
//             }
//         },
//         "post_processors": []
//     "CS:BTC-USD": {
//         "sources": [
//             {
//                 "source_id": "binance",
//                 "id": "btcusdt",
//                 "routes": [
//                     {
//                         "signal_id": "CS:USDT-USD",
//                         "operation": "*"
//                     }
//                 ]
//             },
//             {
//                 "source_id": "coingecko",
//                 "id": "bitcoin",
//                 "routes": []
//             }
//         ],
//         "processor": {
//             "function": "median",
//             "params": {
//                 "min_source_count": 1
//             }
//         },
//         "post_processors": []
//     },
// }
// The function will return a HashMap with the following entries:
// {
//     "binance": ["btcusdt"],
//     "coingecko": ["bitcoin", "tether"]
// }
pub fn get_source_batched_query_ids(
    registry: &Registry<Valid>,
) -> HashMap<String, HashSet<String>> {
    let mut source_query_ids: HashMap<String, HashSet<String>> = HashMap::new();
    // Seen signal_ids
    let mut seen = HashSet::<String>::new();

    let mut queue = VecDeque::from_iter(registry.signal_ids());
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
    use bothan_lib::registry::Invalid;

    use super::*;

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
