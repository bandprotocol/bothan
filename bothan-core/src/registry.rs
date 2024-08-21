use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;

use crate::registry::signal::Signal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod post_processor;
pub mod processor;
pub mod signal;
pub mod source;

#[derive(Clone, Debug, PartialEq)]
pub struct Invalid;

#[derive(Clone, Debug, PartialEq)]
pub struct Valid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registry<State = Invalid> {
    #[serde(flatten)]
    inner: HashMap<String, Signal>,
    #[serde(skip_serializing, default)]
    _state: PhantomData<State>,
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum ValidationError {
    #[error("Circular dependency detected")]
    CircularDependency,

    #[error("Missing dependency")]
    MissingDependency,
}

impl Registry<Invalid> {
    pub fn validate(self) -> Result<Registry<Valid>, ValidationError> {
        let mut in_degrees: HashMap<String, usize> =
            HashMap::from_iter(self.inner.keys().map(|id| (id.clone(), 0)));
        let mut prerequisites: HashMap<String, HashSet<String>> = HashMap::new();
        for (id, signal) in self.inner.iter() {
            let prerequisites_entry = prerequisites.entry(id.clone()).or_default();

            for source_query in signal.source_queries.iter() {
                for route in source_query.routes.iter() {
                    in_degrees
                        .entry(route.signal_id.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    prerequisites_entry.insert(route.signal_id.clone());
                }
            }
        }

        let mut queue = VecDeque::from_iter(
            in_degrees
                .iter()
                .filter_map(
                    |(id, &in_degree)| {
                        if in_degree == 0 {
                            Some(id)
                        } else {
                            None
                        }
                    },
                )
                .cloned(),
        );

        let mut result = Vec::new();
        while let Some(id) = queue.pop_front() {
            let signal_prerequisites = prerequisites
                .get(&id)
                .ok_or(ValidationError::MissingDependency)?;

            for adj in signal_prerequisites {
                in_degrees.entry(adj.clone()).and_modify(|e| *e -= 1);
                if in_degrees[adj] == 0 {
                    queue.push_back(adj.clone());
                }
            }
            result.push(id);
        }

        if result.len() != self.inner.len() {
            return Err(ValidationError::CircularDependency);
        }

        Ok(Registry {
            inner: self.inner,
            _state: PhantomData,
        })
    }
}

impl Default for Registry<Invalid> {
    fn default() -> Self {
        Registry {
            inner: HashMap::new(),
            _state: PhantomData,
        }
    }
}

impl Registry<Valid> {
    pub fn get(&self, signal_id: &str) -> Option<&Signal> {
        self.inner.get(signal_id)
    }
}

#[cfg(test)]
mod test {
    use crate::registry::{Invalid, Registry, ValidationError};

    fn valid_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"coingecko\",\"id\":\"tether\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    fn invalid_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    fn circular_dependency_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:ETH-USD\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:ETH-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"ethusdt\",\"routes\":[{\"signal_id\":\"CS:BTC-USD\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    #[test]
    fn test_registry_validate_valid_registry() {
        let registry = valid_mock_registry();
        let valid_registry = registry.validate();
        assert!(valid_registry.is_ok());
    }

    #[test]
    fn test_registry_validate_invalid_registry() {
        let registry = invalid_mock_registry();
        let valid_registry = registry.validate();
        assert_eq!(valid_registry, Err(ValidationError::MissingDependency));
    }

    #[test]
    fn test_registry_validate_circular_dependency_registry() {
        let registry = circular_dependency_mock_registry();
        let valid_registry = registry.validate();
        assert_eq!(valid_registry, Err(ValidationError::CircularDependency));
    }
}
