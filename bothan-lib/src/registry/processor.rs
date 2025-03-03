use bincode::{Decode, Encode};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod median;
pub mod weighted_median;

#[derive(Clone, Debug, Error, PartialEq, Serialize, Deserialize)]
#[error("{msg}")]
pub struct ProcessError {
    msg: String,
}

impl ProcessError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        ProcessError { msg: msg.into() }
    }
}

/// The Process enum represents the different types of processors that can be used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
pub enum Processor {
    Median(median::MedianProcessor),
    WeightedMedian(weighted_median::WeightedMedianProcessor),
}

impl Processor {
    /// Returns the name of the processor.
    pub fn name(&self) -> &str {
        match self {
            Processor::Median(_) => "median",
            Processor::WeightedMedian(_) => "weighted_median",
        }
    }

    /// Runs the processor on the given data.
    pub fn process(&self, data: Vec<(String, Decimal)>) -> Result<Decimal, ProcessError> {
        match self {
            Processor::Median(median) => {
                let data = data.into_iter().map(|(_, value)| value).collect();
                median.process(data)
            }
            Processor::WeightedMedian(weighted_median) => weighted_median.process(data),
        }
    }
}
