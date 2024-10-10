use bincode::{Decode, Encode};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod median;
pub mod weighted_median;

#[derive(Debug, Error, PartialEq, Clone)]
#[error("{msg}")]
pub struct ProcessError {
    msg: String,
}

impl ProcessError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        ProcessError { msg: msg.into() }
    }
}

/// The Processor trait defines the methods that a processor must implement.
pub trait Process<T, U> {
    fn process(&self, data: Vec<T>) -> Result<U, ProcessError>;
}

/// The Process enum represents the different types of processors that can be used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
pub enum Processor {
    Median(median::MedianProcessor),
    WeightedMedian(weighted_median::WeightedMedianProcessor),
}

impl Process<Decimal, Decimal> for Processor {
    fn process(&self, data: Vec<Decimal>) -> Result<Decimal, ProcessError> {
        match self {
            Processor::Median(median) => median.process(data),
            Processor::WeightedMedian(_) => Err(ProcessError::new(
                "Weighted median not implemented for T: Decimal",
            )),
        }
    }
}

impl Process<(String, Decimal), Decimal> for Processor {
    fn process(&self, data: Vec<(String, Decimal)>) -> Result<Decimal, ProcessError> {
        match self {
            Processor::Median(median) => {
                let data = data.into_iter().map(|(_, value)| value).collect();
                median.process(data)
            }
            Processor::WeightedMedian(weighted_median) => weighted_median.process(data),
        }
    }
}
