use std::collections::HashSet;

use thiserror::Error;

use crate::registry::signal::Signal;
use crate::registry::Registry;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum ValidationError {
    #[error("All signals are part of a cycle")]
    CompleteCycleDetected,

    #[error("Signal {0} contains a cycle")]
    CycleDetected(String),

    #[error("Signal {0} contains an invalid dependency")]
    InvalidDependency(String),
}

pub(crate) fn dfs(
    signal_id: &str,
    signal: &Signal,
    visited: &mut HashSet<String>,
    registry: &Registry,
) -> Result<(), ValidationError> {
    if visited.contains(signal_id) {
        return Err(ValidationError::CycleDetected(signal_id.to_string()));
    }
    visited.insert(signal_id.to_string());

    for source_query in signal.source_queries.iter() {
        for route in source_query.routes.iter() {
            let prereq_signal_id = &route.signal_id;

            let prereq_signal = registry
                .get(prereq_signal_id)
                .ok_or(ValidationError::InvalidDependency(signal_id.to_string()))?;
            dfs(prereq_signal_id, prereq_signal, visited, registry)?;
        }
    }

    Ok(())
}
