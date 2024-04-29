/// This module contains the IdentityProcessor struct and its associated methods.
/// The IdentityProcessor is a simple processor that returns the input as is.
use serde::{Deserialize, Serialize};

use crate::processor::{Processor, ProcessorError};

/// `IdentityProcessor` gets the first element of prerequisites and returns it as is.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentityProcessor {}

impl Processor for IdentityProcessor {
    /// Gets the first element of prerequisites and returns it as is. If prerequisites does not
    /// contain exactly one element, it returns an error.
    fn process(&self, _: Vec<f64>, prerequisites: Vec<f64>) -> Result<f64, ProcessorError> {
        if prerequisites.len() != 1 {
            return Err(ProcessorError::InvalidPrerequisitesAmount);
        }

        Ok(prerequisites[0])
    }
}
