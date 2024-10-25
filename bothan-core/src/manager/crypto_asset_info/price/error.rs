use crate::registry::post_processor::PostProcessError;
use crate::registry::processor::ProcessError;
use std::collections::HashSet;
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
    FailedToProcessPostSignal(#[from] PostProcessError),
}

#[derive(Debug, Error)]
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
