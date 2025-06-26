//! Signal validation logic for the registry module.
//!
//! This module provides functions and error types for validating signals defined in the registry.
//! Validation includes checks for cyclic dependencies, missing dependencies, and proper configuration
//! of weighted median processors.
//!
//! Signal validation ensures that:
//!
//! - There are no cyclic dependencies among signals.
//! - All signal dependencies exist and are properly defined in the registry.
//! - Weighted median processors have correctly specified source weights.

use std::collections::HashMap;

use thiserror::Error;

use crate::registry::Registry;
use crate::registry::processor::Processor;
use crate::registry::processor::weighted_median::WeightedMedianProcessor;
use crate::registry::source::SourceQuery;

/// Errors returned during the signal validation process.
///
/// This enum defines detailed errors for common signal validation issues such as:
///
/// - Dependency cycles.
/// - Non-existent dependencies.
/// - Improper configuration of weighted median processors.
///
/// These errors provide clear messages that can be displayed to users or logged.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum ValidationError {
    /// Indicates that a cyclic dependency has been detected among signals.
    #[error("Signal {0} contains a cycle")]
    CycleDetected(String),

    /// Indicates a dependency on a signal that does not exist in the registry.
    #[error("Signal {0} contains an invalid dependency")]
    InvalidDependency(String),

    /// Indicates incorrect weighted median processor configuration, specifying missing weights.
    #[error("Signal {0} contains an invalid weighted median processor: {1}")]
    InvalidWeightedMedianProcessor(String, String),
}

/// Tracks visitation state of signals during validation.
///
/// This enum is used internally to detect cycles by marking signals as either currently
/// being validated (`InProgress`) or already validated (`Complete`).
pub enum VisitState {
    /// Currently validating this signal; helps detect cycles.
    InProgress,

    /// Signal validation has been completed without errors.
    Complete,
}

pub(crate) fn validate_signal(
    signal_id: &str,
    visited: &mut HashMap<String, VisitState>,
    registry: &Registry,
) -> Result<(), ValidationError> {
    match visited.get(signal_id) {
        Some(VisitState::InProgress) => {
            return Err(ValidationError::CycleDetected(signal_id.to_string()));
        }
        Some(VisitState::Complete) => {
            return Ok(());
        }
        None => {
            visited.insert(signal_id.to_string(), VisitState::InProgress);
        }
    }

    let signal = registry
        .get(signal_id)
        .ok_or(ValidationError::InvalidDependency(signal_id.to_string()))?;

    if let Processor::WeightedMedian(weighted_median) = &signal.processor {
        validate_weighted_median_weights(weighted_median, signal_id, &signal.source_queries)?;
    }

    for source_query in signal.source_queries.iter() {
        for route in source_query.routes.iter() {
            if !registry.contains(&route.signal_id) {
                return Err(ValidationError::InvalidDependency(signal_id.to_string()));
            }
            validate_signal(&route.signal_id, visited, registry)?;
        }
    }

    visited.insert(signal_id.to_string(), VisitState::Complete);

    Ok(())
}

fn validate_weighted_median_weights(
    weighted_median: &WeightedMedianProcessor,
    signal_id: &str,
    source_queries: &[SourceQuery],
) -> Result<(), ValidationError> {
    source_queries.iter().try_for_each(|source_query| {
        if !weighted_median
            .source_weights
            .contains_key(&source_query.source_id)
        {
            Err(ValidationError::InvalidWeightedMedianProcessor(
                signal_id.to_string(),
                source_query.source_id.to_string(),
            ))
        } else {
            Ok(())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::tests::{complete_circular_dependency_mock_registry, valid_mock_registry};

    #[test]
    fn test_validate_signal() {
        let registry = valid_mock_registry();

        let res = validate_signal("CS:BTC-USD", &mut HashMap::new(), &registry);
        assert!(res.is_ok());
    }

    #[test]
    fn test_validate_signal_with_invalid_signal() {
        let registry = valid_mock_registry();

        let res = validate_signal("CS:DNE-USD", &mut HashMap::new(), &registry);
        assert_eq!(
            res,
            Err(ValidationError::InvalidDependency("CS:DNE-USD".to_string()))
        );
    }

    #[test]
    fn test_validate_signal_with_circular_dependency() {
        let registry = complete_circular_dependency_mock_registry();
        let mut visited = HashMap::new();

        let res = validate_signal("CS:USDT-USD", &mut visited, &registry);
        assert_eq!(
            res,
            Err(ValidationError::CycleDetected("CS:USDT-USD".to_string()))
        );
    }
}
