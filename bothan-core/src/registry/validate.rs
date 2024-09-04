use std::collections::HashMap;

use thiserror::Error;

use crate::registry::Registry;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum ValidationError {
    #[error("Signal {0} contains a cycle")]
    CycleDetected(String),

    #[error("Signal {0} contains an invalid dependency")]
    InvalidDependency(String),
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
            return Err(ValidationError::CycleDetected(signal_id.to_string()))
        }
        Some(VisitState::Complete) => return Ok(()),
        None => {
            visited.insert(signal_id.to_string(), VisitState::InProgress);
        }
    }

    let signal = registry
        .get(signal_id)
        .ok_or(ValidationError::InvalidDependency(signal_id.to_string()))?;

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
