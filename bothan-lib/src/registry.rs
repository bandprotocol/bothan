//! Registry for asset signals and their computation dependencies.
//!
//! This module provides a registry system for managing asset signals and their computational
//! relationships. It defines data structures and validation logic to ensure that signal
//! computations can be performed correctly.
//!
//! The module provides:
//!
//! - The [`Registry`] struct which stores signal definitions and their relationships
//! - Type-level states (`Valid` and `Invalid`) to represent validation status
//! - Validation logic to ensure the registry is consistent and free of cycles
//! - Serialization and deserialization support for registry persistence
//!
//! # Registry Structure
//!
//! The registry is a collection of [`Signal`]s, each identified by a unique ID. Signals can
//! depend on other signals for their computation, forming a directed graph. The validation
//! process ensures this graph is acyclic and all dependencies exist.
//!
//! # Module Organization
//!
//! The registry module is organized into several submodules:
//!
//! - [`signal`] - Defines the structure of signals and their components
//! - [`source`] - Defines data sources for signals
//! - [`processor`] - Defines how to process source data into a signal value
//! - [`post_processor`] - Defines transformations applied after initial processing
//!
//! # Type States
//!
//! The registry uses type states to distinguish between validated and unvalidated registries:
//!
//! - [`Registry<Invalid>`] - A registry that has not been validated or has failed validation
//! - [`Registry<Valid>`] - A registry that has been successfully validated and is cycle free
//!
//! This pattern ensures at compile time that only valid registries can be used in contexts
//! where a valid registry is required.

use std::collections::HashMap;
use std::marker::PhantomData;

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::registry::signal::Signal;
pub use crate::registry::validate::ValidationError;
use crate::registry::validate::validate_signal;

pub mod post_processor;
pub mod processor;
pub mod signal;
pub mod source;
pub(crate) mod validate;

/// Marker type representing an unvalidated registry state.
///
/// This type is used as a type parameter for [`Registry`] to indicate that
/// the registry has not been validated or has failed validation. A registry
/// in the `Invalid` state cannot be used for certain operations that require
/// a valid registry.
#[derive(Clone, Debug, PartialEq)]
pub struct Invalid;

/// Marker type representing a validated registry state.
///
/// This type is used as a type parameter for [`Registry`] to indicate that
/// the registry has been successfully validated. A registry in the `Valid` state
/// is guaranteed to have consistent signal dependencies and no cycles.
#[derive(Clone, Debug, PartialEq)]
pub struct Valid;

/// A collection of signals with their computational dependencies.
///
/// The `Registry` struct serves as the central data structure for managing asset signals.
/// It stores signal definitions indexed by their unique IDs and provides methods for
/// accessing and validating them.
///
/// The `State` type parameter indicates whether the registry has been validated:
/// - [`Registry<Invalid>`] - A registry that has not been validated or has failed validation
/// - [`Registry<Valid>`] - A registry that has been successfully validated
///
/// # Examples
///
/// Creating and validating a registry:
///
/// ```
/// use bothan_lib::registry::{Registry, Invalid, Valid};
/// use serde_json::json;
///
/// // Create a registry from JSON
/// let json_data = json!({
///     "BTC-USD": {
///         "sources": [
///             {
///                 "source_id": "binance",
///                 "id": "btcusdt",
///                 "routes": []
///             }
///         ],
///         "processor": {
///             "function": "median",
///             "params": {
///                 "min_source_count": 1
///             }
///         },
///         "post_processors": []
///     }
/// });
///
/// let registry_json = serde_json::to_string(&json_data).unwrap();
/// let registry: Registry<Invalid> = serde_json::from_str(&registry_json).unwrap();
///
/// // Validate the registry
/// let valid_registry: Registry<Valid> = registry.validate().unwrap();
///
/// // Use the validated registry
/// assert!(valid_registry.contains("BTC-USD"));
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Registry<State = Invalid> {
    #[serde(flatten)]
    inner: HashMap<String, Signal>,
    #[serde(skip)]
    _state: PhantomData<State>,
}

impl Registry<Invalid> {
    /// Validates the registry, ensuring all dependencies exist and there are no cycles.
    ///
    /// This method performs a thorough validation of the registry, checking that:
    /// - All signal dependencies (referenced by signal IDs) exist in the registry
    /// - There are no circular dependencies between signals
    ///
    /// If validation is successful, the registry is converted to the `Valid` state.
    /// If validation fails, an error is returned explaining the validation failure.
    ///
    /// # Returns
    ///
    /// - `Ok(Registry<Valid>)` if validation is successful
    /// - `Err(ValidationError)` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use bothan_lib::registry::{Registry, ValidationError};
    /// use serde_json::json;
    ///
    /// // Create a valid registry
    /// let valid_json = json!({
    ///     "USDT-USD": {
    ///         "sources": [
    ///             {
    ///                 "source_id": "coingecko",
    ///                 "id": "tether",
    ///                 "routes": []
    ///             }
    ///         ],
    ///         "processor": {
    ///             "function": "median",
    ///             "params": {
    ///                 "min_source_count": 1
    ///             }
    ///         },
    ///         "post_processors": []
    ///     }
    /// });
    ///
    /// let valid_registry: Registry = serde_json::from_value(valid_json).unwrap();
    /// assert!(valid_registry.validate().is_ok());
    ///
    /// // Create an invalid registry with a missing dependency
    /// let invalid_json = json!({
    ///     "BTC-USD": {
    ///         "sources": [
    ///             {
    ///                 "source_id": "binance",
    ///                 "id": "btcusdt",
    ///                 "routes": [
    ///                     {
    ///                         "signal_id": "USDT-USD", // This dependency doesn't exist
    ///                         "operation": "*"
    ///                     }
    ///                 ]
    ///             }
    ///         ],
    ///         "processor": {
    ///             "function": "median",
    ///             "params": {
    ///                 "min_source_count": 1
    ///             }
    ///         },
    ///         "post_processors": []
    ///     }
    /// });
    ///
    /// let invalid_registry: Registry = serde_json::from_value(invalid_json).unwrap();
    /// assert!(invalid_registry.validate().is_err());
    /// ```
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
    /// Returns the signal for a given signal id.
    ///
    /// This method retrieves a reference to the [`Signal`] with the specified ID,
    /// if it exists in the registry.
    ///
    /// # Returns
    ///
    /// * `Some(&Signal)` if the signal exists in the registry
    /// * `None` if the signal does not exist
    pub fn get(&self, signal_id: &str) -> Option<&Signal> {
        self.inner.get(signal_id)
    }

    /// Returns `true` if the registry contains the given signal id.
    ///
    /// This method checks whether a signal with the specified ID exists in the registry.
    ///
    /// # Returns
    ///
    /// * `true` if the signal exists in the registry
    /// * `false` if the signal does not exist
    pub fn contains(&self, signal_id: &str) -> bool {
        self.inner.contains_key(signal_id)
    }

    /// An iterator visiting all signal ids in the registry in arbitrary order.
    ///
    /// This method returns an iterator over all signal IDs in the registry.
    /// The order of iteration is not specified and may vary between calls.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to signal IDs
    pub fn signal_ids(&self) -> impl Iterator<Item = &String> {
        self.inner.keys()
    }

    /// An iterator visiting all the signal ids and their signals in the registry in arbitrary order.
    ///
    /// This method returns an iterator over all signal IDs and their corresponding signals
    /// in the registry. The order of iteration is not specified and may vary between calls.
    ///
    /// # Returns
    ///
    /// An iterator yielding pairs of references to signal IDs and their signals
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Signal)> {
        self.inner.iter()
    }
}

impl Default for Registry<Valid> {
    /// Creates a new empty validated registry.
    ///
    /// This method creates a default validated registry, which is empty.
    /// This is useful as a starting point for building a registry programmatically.
    ///
    /// # Returns
    ///
    /// An empty `Registry<Valid>`
    fn default() -> Self {
        Registry {
            inner: HashMap::new(),
            _state: PhantomData,
        }
    }
}

impl<State> Encode for Registry<State> {
    /// Encodes the registry for binary serialization.
    ///
    /// This implementation enables efficient binary serialization of the registry
    /// using the `bincode` crate. It encodes the inner map of signals.
    ///
    /// # Errors
    ///
    /// Returns an `EncodeError` if encoding fails
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.inner.encode(encoder)
    }
}

impl<Context> Decode<Context> for Registry<Invalid> {
    /// Decodes a registry from binary serialization.
    ///
    /// This implementation enables efficient binary deserialization of the registry
    /// using the `bincode` crate. It decodes the inner map of signals and creates
    /// an invalid registry that must be validated before use.
    ///
    /// # Errors
    ///
    /// Returns a `DecodeError` if decoding fails
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
