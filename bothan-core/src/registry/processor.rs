use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod median;

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
pub trait Process<T> {
    fn process(&self, data: Vec<T>) -> Result<T, ProcessError>;
}

/// The Process enum represents the different types of processors that can be used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "function", content = "params")]
pub enum Processor {
    Median(median::MedianProcessor),
}

impl Process<Decimal> for Processor {
    fn process(&self, data: Vec<Decimal>) -> Result<Decimal, ProcessError> {
        match self {
            Processor::Median(median) => median.process(data),
        }
    }
}
