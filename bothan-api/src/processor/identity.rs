use std::str::FromStr;

use serde::{Deserialize, Serialize};

use bothan_core::service::ServiceResult;
use bothan_core::types::PriceData as SourcePriceData;

use crate::processor::{Processing, ProcessingError};
use crate::proto::query::query::PriceData;

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
