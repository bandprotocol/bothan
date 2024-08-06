use thiserror::Error;

use crate::registry::post_processor::PostProcessorError;
use crate::registry::processor::ProcessError;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum SetRegistryError {
    #[error("Failed to get registry from IPFS")]
    FailedToRetrieve(String),

    #[error("Unsupported version")]
    UnsupportedVersion,

    #[error("Failed to parse into registry")]
    FailedToParse,

    #[error("Registry is invalid")]
    InvalidRegistry,

    #[error("Invalid ipfs hash")]
    InvalidHash,
}

#[derive(Debug, Error, PartialEq, Clone)]
#[error("Signal {signal_id} doesnt exist in the registry")]
pub struct MissingSignalError {
    pub signal_id: String,
}

#[derive(Debug, Error, PartialEq, Clone)]
pub enum GetPriceError {
    #[error("Failed to create registry: {0}")]
    RegistryCreation(String),

    #[error("Failed to create tasks: {0}")]
    TaskCreation(String),

    #[error("Failed to execute tasks: {0}")]
    TaskExecution(String),
}

#[derive(Debug, Error, PartialEq, Clone)]
pub enum SourceRoutingError {
    #[error("Missing source result for key {0}")]
    MissingSource(String),

    #[error("Could not find signal id prerequisites for key {0}")]
    IncompletePrerequisites(String),
}

#[derive(Debug, Error, PartialEq, Clone)]
pub enum SignalTaskError {
    #[error("Failed to compute signal ID: {0}")]
    FailedSourceRouting(#[from] SourceRoutingError),

    #[error("Failed to process signal task: {0}")]
    FailedProcessExecution(#[from] ProcessError),

    #[error("Failed to post process signal task: {0}")]
    FailedPostProcessExecution(#[from] PostProcessorError),
}
