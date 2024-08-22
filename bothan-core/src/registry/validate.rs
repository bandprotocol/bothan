use std::collections::HashMap;

use thiserror::Error;

use crate::registry::signal::Signal;
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

pub(crate) fn dfs(
    signal_id: &str,
    signal: &Signal,
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

    for source_query in signal.source_queries.iter() {
        for route in source_query.routes.iter() {
            let prereq_signal_id = &route.signal_id;

            let prereq_signal = registry
                .get(prereq_signal_id)
                .ok_or(ValidationError::InvalidDependency(signal_id.to_string()))?;
            dfs(prereq_signal_id, prereq_signal, visited, registry)?;
        }
    }

    visited.insert(signal_id.to_string(), VisitState::Complete);

    Ok(())
}
