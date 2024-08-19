use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::registry::signal::Signal;

pub mod post_processor;
pub mod processor;
pub mod signal;
pub mod source;

#[derive(Clone, Debug)]
pub struct Invalid;

#[derive(Clone, Debug)]
pub struct Valid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Registry<State = Invalid> {
    #[serde(flatten)]
    inner: HashMap<String, Signal>,
    #[serde(skip_serializing, default)]
    _state: PhantomData<State>,
}

impl Registry<Invalid> {
    pub fn validate(self) -> Result<Registry<Valid>, Self> {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from_iter(self.inner.keys().cloned());

        while let Some(signal_id) = queue.pop_front() {
            if seen.contains(&signal_id) {
                continue;
            }

            match self.inner.get(&signal_id) {
                None => return Err(self),
                Some(signal) => {
                    let mut missing_pid_flag = false;
                    for source in &signal.source_queries {
                        for route in &source.routes {
                            if !seen.contains(&route.signal_id) {
                                queue.push_back(route.signal_id.clone());
                                missing_pid_flag = true;
                            }
                        }
                    }

                    if missing_pid_flag {
                        queue.push_back(signal_id);
                    } else {
                        seen.insert(signal_id);
                    }
                }
            }
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
    use crate::registry::{Invalid, Registry};

    fn valid_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"coingecko\",\"id\":\"tether\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    fn invalid_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
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
        assert!(valid_registry.is_err());
    }
}
