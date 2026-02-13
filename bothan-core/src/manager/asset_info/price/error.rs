//! Error types for price computation in the crypto asset info manager.

use std::collections::HashSet;

use bothan_lib::registry::post_processor::PostProcessError;
use bothan_lib::registry::processor::ProcessError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Signal does not exist")]
    InvalidSignal,

    #[error("{0}")]
    PrerequisiteRequired(#[from] MissingPrerequisiteError),

    #[error("Failed to process signal: {0}")]
    FailedToProcessSignal(#[from] ProcessError),

    #[error("Failed to post process signal: {0}")]
    FailedToPostProcessSignal(#[from] PostProcessError),
}

#[derive(Debug, Error, PartialEq)]
#[error("Missing prerequisite: {ids:?}")]
pub struct MissingPrerequisiteError {
    pub ids: Vec<String>,
}

impl MissingPrerequisiteError {
    pub fn new(ids: Vec<String>) -> Self {
        MissingPrerequisiteError { ids }
    }
}

impl From<HashSet<String>> for MissingPrerequisiteError {
    fn from(value: HashSet<String>) -> Self {
        MissingPrerequisiteError {
            ids: value.into_iter().collect(),
        }
    }
}
