use serde::{Deserialize, Serialize};

use crate::processor::{Processing, ProcessingError};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentityProcessor {}

impl Processing for IdentityProcessor {
    fn process(&self, _: Vec<f64>, prerequisites: Vec<f64>) -> Result<f64, ProcessingError> {
        if prerequisites.len() != 1 {
            return Err(ProcessingError::InvalidPrerequisitesAmount);
        }

        Ok(prerequisites[0])
    }
}
