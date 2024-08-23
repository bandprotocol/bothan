use std::collections::HashMap;
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
        let mut visited = HashMap::new();
        for root in self.inner.keys() {
            dfs(root, &mut visited, &self)?;
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
    pub fn contains(&self, signal_id: &str) -> bool {
        self.inner.contains_key(signal_id)
    }
}

#[cfg(test)]
mod test {
    use crate::registry::{Invalid, Registry, ValidationError};

    fn valid_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"A\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1a\",\"routes\":[{\"signal_id\":\"C\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"B\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1b\",\"routes\":[{\"signal_id\":\"C\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"C\":{\"sources\":[{\"source_id\":\"source_1\",\"id\":\"s1c\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
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
        assert_eq!(
            registry.validate(),
            Err(ValidationError::InvalidDependency("A".to_string()))
        );
    }

    #[test]
    fn test_registry_validate_registry_with_complete_circular_dependency() {
        let registry = complete_circular_dependency_mock_registry();
        assert!(matches!(
            registry.validate(),
            Err(ValidationError::CycleDetected(_))
        ))
    }

    #[test]
    fn test_registry_validate_registry_with_circular_dependency() {
        let registry = circular_dependency_mock_registry();
        assert!(matches!(
            registry.validate(),
            Err(ValidationError::CycleDetected(_))
        ))
    }
}
