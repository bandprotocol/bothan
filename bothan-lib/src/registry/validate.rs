use std::collections::HashMap;

use thiserror::Error;

use crate::registry::Registry;
use crate::registry::processor::{Processor, weighted_median::WeightedMedianProcessor};
use crate::registry::source::SourceQuery;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum ValidationError {
    #[error("Signal {0} contains a cycle")]
    CycleDetected(String),

    #[error("Signal {0} contains an invalid dependency")]
    InvalidDependency(String),

    #[error("Signal {0} contains an invalid weighted median processor: {1}")]
    InvalidWeightedMedianProcessor(String, String),
}

pub enum VisitState {
    InProgress,
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
        Some(VisitState::Complete) => return Ok(()),
        None => {
            visited.insert(signal_id.to_string(), VisitState::InProgress);
        }
    }

    let signal = registry
        .get(signal_id)
        .ok_or(ValidationError::InvalidDependency(signal_id.to_string()))?;

    match signal.processor.clone() {
        Processor::WeightedMedian(weighted_median) => {
            validate_weighted_median_weights(&weighted_median, signal_id, &signal.source_queries)?;
        }
        _ => {}
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
            return Err(ValidationError::InvalidWeightedMedianProcessor(
                signal_id.to_string(),
                source_query.source_id.to_string(),
            ));
        }
        Ok(())
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
