use std::collections::HashMap;
use std::marker::PhantomData;

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::registry::signal::Signal;
use crate::registry::validate::validate_signal;
pub use crate::registry::validate::ValidationError;

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
    #[serde(skip)]
    _state: PhantomData<State>,
}

impl Registry<Invalid> {
    pub fn validate(self) -> Result<Registry<Valid>, ValidationError> {
        let mut visited = HashMap::new();
        for root in self.inner.keys() {
            validate_signal(root, &mut visited, &self)?;
        }

        Ok(Registry {
            inner: self.inner,
            _state: PhantomData,
        })
    }
}

impl<T> Registry<T> {
    pub fn get(&self, signal_id: &str) -> Option<&Signal> {
        self.inner.get(signal_id)
    }
    pub fn contains(&self, signal_id: &str) -> bool {
        self.inner.contains_key(signal_id)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.inner.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Signal)> {
        self.inner.iter()
    }
}

impl Default for Registry<Valid> {
    fn default() -> Self {
        Registry {
            inner: HashMap::new(),
            _state: PhantomData,
        }
    }
}

impl<State> Encode for Registry<State> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.inner.encode(encoder)
    }
}

impl Decode for Registry<Invalid> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let inner = HashMap::decode(decoder)?;
        Ok(Registry {
            inner,
            _state: PhantomData,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::registry::{Invalid, Registry, ValidationError};

    pub fn valid_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"coingecko\",\"id\":\"tether\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    pub fn invalid_dependency_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:BTC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"btcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]},{\"source_id\":\"coingecko\",\"id\":\"bitcoin\",\"routes\":[]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    pub fn complete_circular_dependency_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"usdtusdc\",\"routes\":[{\"signal_id\":\"CS:USDC-USD\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:USDC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"usdcusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
        serde_json::from_str::<Registry>(json_string).unwrap()
    }

    pub fn circular_dependency_mock_registry() -> Registry<Invalid> {
        let json_string = "{\"CS:USDT-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"usdtusdc\",\"routes\":[{\"signal_id\":\"CS:USDC-USD\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:USDC-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"usdcdai\",\"routes\":[{\"signal_id\":\"CS:DAI-USD\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]},\"CS:DAI-USD\":{\"sources\":[{\"source_id\":\"binance\",\"id\":\"daiusdt\",\"routes\":[{\"signal_id\":\"CS:USDT-USD\",\"operation\":\"*\"}]}],\"processor\":{\"function\":\"median\",\"params\":{\"min_source_count\":1}},\"post_processors\":[]}}";
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
            Err(ValidationError::InvalidDependency("CS:BTC-USD".to_string()))
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
