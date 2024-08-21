use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::registry::signal::Signal;
use crate::registry::validate::{dfs, ValidationError};

pub mod post_processor;
pub mod processor;
pub mod signal;
pub mod source;
pub(crate) mod validate;

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

impl Registry<Invalid> {
    pub fn validate(self) -> Result<Registry<Valid>, ValidationError> {
        // If the registry is empty, there are no dependencies to validate
        if self.inner.is_empty() {
            return Ok(Registry {
                inner: self.inner,
                _state: PhantomData,
            });
        }

        // Find the in degrees of each signal
        let mut in_degrees: HashMap<String, usize> =
            HashMap::from_iter(self.inner.keys().map(|id| (id.clone(), 0)));
        for (_, signal) in self.inner.iter() {
            for source_query in signal.source_queries.iter() {
                for route in source_query.routes.iter() {
                    in_degrees
                        .entry(route.signal_id.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }

        // Use the signals with 0 in degrees as the roots
        let roots = in_degrees
            .iter()
            .filter(|(_, &in_degree)| in_degree == 0)
            .collect::<Vec<_>>();

        // If there are no roots take the first signal as the issue
        if roots.is_empty() {
            return Err(ValidationError::CompleteCycleDetected);
        }

        let mut visited = HashSet::new();
        for (root, _) in roots {
            let signal = self
                .inner
                .get(root)
                .ok_or(ValidationError::InvalidDependency(root.clone()))?;

            dfs(root, signal, &mut visited, &self)?;
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

impl<T> Registry<T> {
    pub fn get(&self, signal_id: &str) -> Option<&Signal> {
        self.inner.get(signal_id)
    }
}

#[cfg(test)]
mod test {
    use crate::registry::{Invalid, Registry, ValidationError};

    fn valid_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"A\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1a\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"B\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1b\",\"routes\":[{\"signal_id\":\"A\",\"operation\":\"*\"}]},{\"source_id\":\"source_2\",\"id\":\"s2b\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    fn invalid_dependency_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"A\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1a\",\"routes\":[{\"signal_id\":\"B\",\"operation\":\"*\"}]},{\"source_id\":\"source_2\",\"id\":\"s2a\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    fn complete_circular_dependency_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"A\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1a\",\"routes\":[{\"signal_id\":\"B\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"B\":{\"sources\":[{\"source_id\":\"source_2\",\"id\":\"s2b\",\"routes\":[{\"signal_id\":\"A\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    fn circular_dependency_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"A\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1a\",\"routes\":[{\"signal_id\":\"B\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"B\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1b\",\"routes\":[{\"signal_id\":\"C\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"C\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1c\",\"routes\":[{\"signal_id\":\"B\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    #[test]
    fn test_registry_validate_valid_registry() {
        let registry = valid_mock_registry();
        let valid_registry = registry.validate();
        assert!(valid_registry.is_ok());
    }

    #[test]
    fn test_registry_validate_registry_with_invalid_dependency() {
        let registry = invalid_dependency_mock_registry();
        let valid_registry = registry.validate();
        assert_eq!(
            valid_registry,
            Err(ValidationError::InvalidDependency("B".to_string()))
        );
    }

    #[test]
    fn test_registry_validate_registry_with_complete_circular_dependency() {
        let registry = complete_circular_dependency_mock_registry();
        let valid_registry = registry.validate();
        assert_eq!(valid_registry, Err(ValidationError::CompleteCycleDetected));
    }

    #[test]
    fn test_registry_validate_registry_with_circular_dependency() {
        let registry = circular_dependency_mock_registry();
        let valid_registry = registry.validate();
        assert_eq!(
            valid_registry,
            Err(ValidationError::CycleDetected("B".to_string()))
        );
    }
}
