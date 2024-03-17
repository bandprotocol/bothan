use serde::{Deserialize, Serialize};

use crate::processor::{Processor, ProcessorError};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentityProcessor {}

impl Processor for IdentityProcessor {
    fn process(&self, _: Vec<f64>, prerequisites: Vec<f64>) -> Result<f64, ProcessorError> {
        if prerequisites.len() != 1 {
            return Err(ProcessorError::InvalidPrerequisitesAmount);
        }

        Ok(prerequisites[0])
    }
}
